"""
This module contains clean algorithm templates for tree data structures.
"""
from __future__ import annotations

class BinaryTreeNode(object):
    """
    This class act as a single basis for binary tree node
    """
    def __init__(self, key=0):
        self._lchild = self.rchild = self._parent = None
        self._key = key

    @property
    def left_child(self) -> BinaryTreeNode:
        return self._lchild

    @left_child.setter
    def left_child(self, node: BinaryTreeNode):
        # This method will assign parent pointer automatically
        assert node != self, "You are creating a reference loop!"
        self._lchild = node
        if node: node._parent = self

    @property
    def right_child(self) -> BinaryTreeNode:
        return self._rchild

    @right_child.setter
    def right_child(self, node: BinaryTreeNode):
        # This method will assign parent pointer automatically
        assert node != self, "You are creating a reference loop!"
        self._rchild = node
        if node: node._parent = self

    @property
    def parent(self) -> BinaryTreeNode:
        return self._parent

    @property
    def key(self):
        return self.key

    # ------------------------ Balance operations -----------------------

    def update_down(self):
        """ This method update stored value downward """
        pass

    def update_up(self):
        """ This method update stored value upward """
        pass

    def _transfer_parent(self, node: BinaryTreeNode):
        if self.parent.left_child == self:
            self.parent.left_child = node
        else:
            self.parent.right_child = node

    def zig(self):
        """
        Right Rotation at the current position
            c.zig() =>
                p            c  
               / \          / \ 
              c   z        x   p
             / \      ->      / \ 
            x   y            y   z
        """
        parent = self.parent
        self.parent.update_down()

        parent.left_child = self.right_child
        parent._transfer_parent(self)
        self.right_child = parent

        parent.update_up()

    def zag(self):
        """
        Left Rotation at the current position
            c.zag() =>
               p                  c  
              / \                / \ 
             x   c              p   z
                / \     ->     / \ 
               y   z          x   y
        """
        parent = self.parent
        self.parent.update_down()

        parent.right_child = self.left_child
        parent._transfer_parent(self)
        self.left_child = parent

        parent.update_up()

    def _connect34(self, a: BinaryTreeNode, b: BinaryTreeNode, c: BinaryTreeNode,
        t0: BinaryTreeNode, t1: BinaryTreeNode, t2: BinaryTreeNode, t3: BinaryTreeNode. top:BinaryTreeNode):
        """
        Reconstruct connected node a,b,c and their childrens. Structure after this operation is
                 b
               /   \
              a     c
             / \   / \
            t0 t1  t2 t3
        """
        top._transfer_parent(b)

        a.left_child = t0
        a.right_child = t1
        # TODO(template): Add upward update on a here if needed

        c.left_child = t2
        c.right_child = t3
        # TODO(template): Add upward update on b here if needed

        b.left_child = a
        b.right_child = c
        # TODO(template): Add upward update on c here if needed

    def balance3(self):
        """
        make current node and its parent and grandparent balanced
              g          g          g         g
             / \        / \        / \       / \ 
            p   t3     p   t3     t0  p     t0  p 
           / \        / \            / \       / \ 
          v   t2     t0  v          v   t3    t1  v 
         / \            / \        / \           / \ 
        t0 t1          t1 t2      t1 t2         t2 t3
          case 1      case 2      case 3     case 4
        """
        v = self
        p = v.parent
        g = p.parent
        if p == g.left_child:
            if v == p.left_child: # case 1
                return self._connect34(v, p, g, v.left_child, v.right_child, p.right_child, g.right_child, g) # zig
            else: # case 2
                return self._connect34(p, v, g, p.left_child, v.left_child, v.right_child, g.right_child, g) # zag-zig
        else:
            if v == p.left_child: # case 3
                return self._connect34(g, v, p, g.left_child, v.left_child, v.right_child, p.right_child, g) # zig-zag
            else: # case 4
                return self._Connect34(g, p, v, g.left_child, p.left_child, v.left_child, v.right_child, g) # zig

class BinarySearchTree(object):
    _dummy_root = BinaryTreeNode()

    def __init__(self, distinct_key=True):
        self._root_sentinel = BinaryTreeNode()
        self._count = 0
        self._distinct = distinct_key
    
    @property
    def root(self) -> BinaryTreeNode:
        return self._root_sentinel.right_child

    @root.setter
    def root(self, node: BinaryTreeNode):
        self._root_sentinel.right_child = node

    @property
    def count(self) -> int:
        return self.count

    def clear(self):
        self.root = None
        self._count = 0

    def _transplant(self, target: BinaryTreeNode, replace: BinaryTreeNode):
        """ Replace target node in the tree """
        target.parent.update_down()
        target._transfer_parent(replace)
        replace.update_up()

    def find(self, key, exact=True) -> BinaryTreeNode:
        """ find for a node in the tree given key """

        current = self.root
        ret = None
        while current != None:
            result = _comparer.Compare(current.Key, key);
            if current.Key == key:
                if not self._distinct
                    return current
                else
                    ret = current

            if current.Key < key:
                current = current.right_child # search into right subtree
            else: # keep left first when keep insert order
                current = current.left_child # search into left subtree

        if ret or exact:
            return ret
        else:
            return current

    def __iter__(self):
        pass

class PersistentBinaryTreeNode(BinaryTreeNode):
    """ This binary tree add version to each node """
    pass

class AVLTreeNode(BinaryTreeNode):
    """
    This class represent a node in an AVL tree
    """
    def __init__(self):
        super().__init__()
        self._height = 1

    @property
    def left_height(self) -> int:
        return self.left_child._height if self.left_child is not None else 0

    @property
    def right_height(self) -> int:
        return self.right_child._height if self.left_child is not None else 0

    @property
    def balance_factor(self) -> int:
        return self.left_height - self.right_height

    @property
    def taller_child(self) -> AVLTreeNode:
        if self.balance_factor > 0:
            return self.left_height
        else:
            return self.right_height

class CompactBinaryTreeNode(object):
    """ This class represent a binary tree stored in an array """
    pass
