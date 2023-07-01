from typing import List
from pygltflib import GLTF2, Node
from pygltflib.validator import validate, summary

MDL_GLTF: str = 'kid.gltf'
WPN_GLTF: str = 'wpn.gltf'


def travel(nodes: List[Node], index: int, level: int = 0):
    node = nodes[index]
    for i in range(level):
        print('    ', sep='', end='')
    print(f'{index}  {node.name}  {node.children}')

    level += 1
    for child in node.children:
        travel(nodes, child, level)
    level -= 1
    

def main():
    gltf = GLTF2().load(MDL_GLTF)
    validate(gltf)
    summary(gltf)

    scene = gltf.scenes[gltf.scene]
    nodes = gltf.nodes

    # for index, node in enumerate(nodes):
    #     print(f'index:{index}  name:{node.name}  children:{node.children}')

    travel(nodes, 40)



if __name__ == '__main__':
    main()