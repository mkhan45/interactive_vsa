window.root = document.querySelector('#root');
window.vsas = () => allChildren(root);
let node_widths = new Map();

const scene_node = document.querySelector('.scene');

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

let update_lines = node => {
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
}

let move_node = (node, x, y) => {
    node.style.left = x + 'px';
    node.style.top = y + 'px';
    update_lines(node);
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

            if (box.parentNode.classList.contains('union')) {
                let btn = document.createElement('button');
                btn.innerText = 'Select';
                btn.onclick = () => select_from_union(child);
                btn.id = 'select';
                node.appendChild(btn);
            }
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
            let x = e.clientX - parseInt(scene_node.style.left) + window.scrollX;
            let y = e.clientY - parseInt(scene_node.style.top) + window.scrollY;
            let move = (e) => {
                x = e.clientX - parseInt(scene_node.style.left) + window.scrollX;
                y = e.clientY - parseInt(scene_node.style.top) + window.scrollY;
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
    //console.log(children, total_branch_width, x, y, child_x, child_y);
    for (let child of children) {
        let branch_width = node_widths.get(child);
        place(child, child_x, child_y);
        child_x += branch_width;
    }
}

//place(root_vsa, window.innerWidth / 2 - root_vsa.offsetWidth / 2, 100);

window.learn_depth = 1;

function learn(el, arg) {
    let root = el.parentNode;
    let root_pos_x = root.offsetLeft;
    let root_pos_y = root.offsetTop;
    let vsa_html = wasm_bindgen.learn(arg, learn_depth);

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
 
    let min_x_dist = -15 + Math.min(...[...document.body.querySelectorAll("*")].map(x => x.getBoundingClientRect().x));
    let max_x_dist = 15 + Math.max(...[...document.body.querySelectorAll("*")].map(x => x.getBoundingClientRect().x));

    /*
    scene_node.style.transform =
      "translateX(" +
      -(
        -15 +
        Math.min(
          ...[...document.body.querySelectorAll("*")].map(
            (x) => x.getBoundingClientRect().x
          )
        )
      ) +
      "px)"; 
    */
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
        scene_node.style.left = parseInt(scene_node.style.left) + 50 + 'px';
    } else if (e.key === 'ArrowRight') {
        scene_node.style.left = parseInt(scene_node.style.left) - 50 + 'px';
    } else if (e.key === 'ArrowUp') {
        scene_node.style.top = parseInt(scene_node.style.top) + 50 + 'px';
    } else if (e.key === 'ArrowDown') {
        scene_node.style.top = parseInt(scene_node.style.top) - 50 + 'px';
    }
}

window.mode = 'base';

function set_mode_base() {
    window.mode = 'base';
    document.querySelector('#select-status').innerHTML = 'Off';

    document.querySelectorAll('.box').forEach(el => el.onclick = e => {})
}

function removeNode(node) {
    node.parentNode.removeChild(node);

    for (let child of nodes.get(node).children) {
        removeNode(child);
    }

    let parent = nodes.get(node).parents.values().next().value;
    let parent_info = nodes.get(parent);
    nodes.get(parent).from_lines = [...parent_info.from_lines].filter(l => !nodes.get(node).to_lines.has(l));

    let from_lines = nodes.get(node).from_lines;
    let to_lines = nodes.get(node).to_lines;
    for (let line of [...from_lines, ...to_lines]) {
        svg_container.removeChild(line);
    }

    nodes.delete(node);
}

function select_from_union(el) {
    let select_btn = el.querySelector('#select');
    el.removeChild(select_btn);

    update_lines(el);

    let parent = nodes.get(el).parents.values().next().value;
    if (!parent.parentNode.classList.contains('union')) {
        return;
    }

    let siblings = [...nodes.get(parent).children].filter(c => c !== el);
    for (let sibling of siblings) {
        removeNode(sibling);
    }
}
