window.root = document.querySelector('#root');
window.vsas = () => allChildren(root);
let node_widths = new Map();

function allChildren(node) {
    function helper(node, acc) {
        for (let child of node.children) {
            acc.push(child);
            helper(child, acc);
        }
    }

    let acc = [node]
    helper(node, acc);
    return acc;
}

/*
vsas().forEach(node => {
    node.onclick = () => {
        console.log(node);
    }
});
*/

let svg_container = document.querySelector('svg');
let nodes = new Map();
let init_node = node => {
    nodes.set(node, {
        node: node,
        children: new Set(),
        parents: new Set(),
        from_lines: new Set(),
        to_lines: new Set()
    });
};
let If = cond => { return { then: eff => cond && eff() } };
let ensure_init = node => If(nodes.get(node) === undefined).then(() => init_node(node));
let add_child = (parent, child) => {
    ensure_init(parent); // probably redundant
    return nodes.get(parent).children.add(child);
};
let add_parent = (child, parent) => {
    ensure_init(child);
    nodes.get(child).parents.add(parent);
};
let add_edge = (parent, child, line) => {
    add_child(parent, child);
    add_parent(child, parent);
    nodes.get(parent).from_lines.add(line);
    nodes.get(child).to_lines.add(line);
};
let move_node = (node, x, y) => {
    node.style.left = x + 'px';
    node.style.top = y + 'px';
    let node_info = nodes.get(node);
    let from_lines = node_info.from_lines;
    let to_lines = node_info.to_lines;
    for (let line of from_lines) {
        line.setAttribute('x1', node.offsetLeft + node.offsetWidth / 2);
        line.setAttribute('y1', node.offsetTop + node.offsetHeight);
    }
    for (let line of to_lines) {
        line.setAttribute('x2', node.offsetLeft + node.offsetWidth / 2);
        line.setAttribute('y2', node.offsetTop);
    }
};
let root_vsa = null;
function find_parent_box(node) {
    function helper(node) {
        if (node.classList === undefined) return null;

        if (node.classList.contains('union') || node.classList.contains('join')) {
            return node.children[0];
        } else {
            return helper(node.parentNode);
        }
    }

    return helper(node.parentNode.parentNode);
}

function graphify(root) {
    console.log(root);
    for (let node of root.querySelectorAll('.box')) {
        let box = find_parent_box(node);
        if (box !== null) {
            let child = node;
            let line = svg_container.appendChild(document.createElementNS('http://www.w3.org/2000/svg', 'line'));
            line.setAttribute('x1', box.offsetLeft + box.offsetWidth / 2);
            line.setAttribute('y1', box.offsetTop + box.offsetHeight);
            line.setAttribute('x2', child.offsetLeft + child.offsetWidth / 2);
            line.setAttribute('y2', child.offsetTop);
            line.setAttribute('stroke', 'black');

            add_edge(box, child, line);
        } else {
            // root
            root_vsa = node;
            init_node(node);
        }
    }

    for (let [node, info] of nodes.entries()) {
        let from_lines = Array.from(info.from_lines);
        let to_lines = Array.from(info.to_lines);

        // make node draggable
        node.onmousedown = (e) => {
            let x = e.clientX;
            let y = e.clientY;
            let move = (e) => {
                x = e.clientX - parseInt(document.body.style.left);
                y = e.clientY - parseInt(document.body.style.top);
                move_node(node, x - node.offsetWidth / 2, y - node.offsetHeight / 2);
            };
            let up = (e) => {
                document.removeEventListener('mousemove', move);
                document.removeEventListener('mouseup', up);
            };
            document.addEventListener('mousemove', move);
            document.addEventListener('mouseup', up);
        };
    }
}


// might need to start from the bottom up
function get_node_width(node) {
    let children = nodes.get(node).children;
    let child_widths = [...children].map(c => get_node_width(c));
    let total_child_width = child_widths.reduce((x, y) => x + y, 0);
    if (total_child_width === 0) {
        node_widths.set(node, node.offsetWidth + 15);
    } else {
        node_widths.set(node, Math.max(total_child_width, node.offsetWidth + 15));
    }

    return node_widths.get(node);
}

//get_node_width(root_vsa);
function place(node, x, y) {
    get_node_width(node);
    move_node(node, x, y);
    let center_x = x + node.offsetWidth / 2;

    let children = nodes.get(node).children;
    let child_widths = [...children].map(c => node_widths.get(c));
    let total_branch_width = child_widths.reduce((x, y) => x + y, 0);
    let child_x = center_x - total_branch_width / 2;
    let branch_height = node.offsetHeight * 1.5;
    let child_y = y + branch_height;
    console.log(children, total_branch_width, x, y, child_x, child_y);
    for (let child of children) {
        let branch_width = node_widths.get(child);
        place(child, child_x, child_y);
        child_x += branch_width;
    }
}

//place(root_vsa, window.innerWidth / 2 - root_vsa.offsetWidth / 2, 100);

function learn(el, arg) {
    let root = el.parentNode;
    let root_pos_x = root.offsetLeft;
    let root_pos_y = root.offsetTop;
    let vsa_html = wasm_bindgen.learn(arg);

    let wrapper_node = document.createElement('div');
    wrapper_node.innerHTML = vsa_html;
    let vsa_node = wrapper_node.children[0];
    //console.log(vsa_node);
    //root.replaceChild(vsa_node, el);

    let root_from_lines = nodes.get(root).from_lines;
    let root_to_lines = nodes.get(root).to_lines;
    for (let line of [...root_from_lines, ...root_to_lines]) {
        svg_container.removeChild(line);
    }
    nodes.delete(root);
    root.parentNode.replaceChild(vsa_node, root);
    // TODO: place properly

    graphify(vsa_node);
    place(vsa_node.querySelector(".box"), root_pos_x, root_pos_y);
}

window.onload = () => {
    wasm_bindgen().then(() => {
        let initial = document.querySelector('.unlearned');
        let mid_x = window.innerWidth / 2 - initial.offsetWidth / 2;
        graphify(initial.parentNode);
        place(initial, mid_x, 100);
    })
}

document.body.onkeydown = (e) => {
    if (e.key === 'ArrowLeft') {
        document.body.style.left = parseInt(document.body.style.left) + 10 + 'px';
    } else if (e.key === 'ArrowRight') {
        document.body.style.left = parseInt(document.body.style.left) - 10 + 'px';
    } else if (e.key === 'ArrowUp') {
        document.body.style.top = parseInt(document.body.style.top) + 10 + 'px';
    } else if (e.key === 'ArrowDown') {
        document.body.style.top = parseInt(document.body.style.top) - 10 + 'px';
    }
}
