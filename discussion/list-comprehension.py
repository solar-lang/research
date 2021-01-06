
def inorder(t):
    if not t:
        return []

    return [for l in inorder(t.left), t.val, r for r in inorder(t.right)]

def preorder(t):
    if not t:
        return []
    return [t.val, l for l in inorder(t.left), r for r in inorder(t.right)]


# THIS LOOKS LIKE SHIT UP THERE

# this looks MUCH better

def inorder(t):
    if not t:
        return []

    return inorder(t.left) + [t.val] + inorder(t.right)

def preorder(t):
    if not t:
        return []
    return [t.val] + preorder(t.left) + preorder(t.right)

def postorder(t):
    if not t:
        return []
    return preorder(t.left) + preorder(t.right) + [t.val]
