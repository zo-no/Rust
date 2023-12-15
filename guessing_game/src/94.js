var inorderTraversal = function (root) {
  let res = [];
  const inorder = (root, res) => {
    if (!root) return;
    if (root.left) {
      inorder(root.left, res);
    }
    res.push(root.val);
    if (root.right) {
      inorder(root.right, res);
    }
  };
  inorder(root, res);
  return res;
};
