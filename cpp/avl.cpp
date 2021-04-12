#include <cmath>
#include <cstdio>
#include <iostream>

using namespace std;

class Node {
private:
  int key, height;
  Node *left, *right;

public:
  Node(int key) : key(key), height(1), left(nullptr), right(nullptr) {}

  friend class Tree;
};

enum class Order {
  Pre = 0,
  In = 1,
  Post = 2,
};

class Tree {
public:
  Tree() : root(nullptr) {}
  void insert(int key) { root = insert_with_node(root, key); }
  void del(int key) { root = delete_node(root, key); }

  void display(Order order = Order::Pre) {
    switch (order) {
    case Order::Pre: {
      cout << "pre-order:";
      display_pre(root);
      break;
    }
    case Order::In: {
      cout << "in-order:";
      display_in(root);
      break;
    }
    case Order::Post: {
      cout << "post-order:";
      display_post(root);
      break;
    }
    }
    cout << endl;
  }

private:
  void display_node(Node *node) {
    auto key = node->key;
    auto leftk = node->left ? node->left->key : -1;
    auto rightk = node->right ? node->right->key : -1;

    cout << " (key=" << key << ",left=" << leftk << ",right=" << rightk << ")";
  }
  void display_pre(Node *node) {
    if (node != nullptr) {
      display_node(node);
      display_pre(node->left);
      display_pre(node->right);
    }
  }
  void display_in(Node *node) {
    if (node != nullptr) {
      display_pre(node->left);
      display_node(node);
      display_pre(node->right);
    }
  }
  void display_post(Node *node) {
    if (node != nullptr) {
      display_pre(node->left);
      display_pre(node->right);
      display_node(node);
    }
  }

  inline int height(Node *n) {
    if (n == nullptr)
      return 0;
    return n->height;
  }

  inline int max(int a, int b) { return (a > b) ? a : b; }
  inline int max_height(Node *x) {
    return max(height(x->left), height(x->right));
  }

  Node *right_rotate(Node *y) {
    Node *x = y->left;
    Node *T2 = x->right;

    x->right = y;
    y->left = T2;

    y->height = max_height(y) + 1;
    x->height = max_height(x) + 1;
    return x;
  }

  Node *left_rotate(Node *x) {
    Node *y = x->right;
    Node *T2 = y->left;

    y->left = x;
    x->right = T2;

    x->height = max_height(x) + 1;
    y->height = max_height(y) + 1;
    return y;
  }

  int get_balance(Node *n) {
    if (n == nullptr)
      return 0;
    return height(n->left) - height(n->right);
  }

  Node *insert_with_node(Node *node, int key) {
    if (node == nullptr)
      return new Node(key);

    if (key < node->key)
      node->left = insert_with_node(node->left, key);
    else if (key > node->key)
      node->right = insert_with_node(node->right, key);
    else
      return node;

    node->height = 1 + max_height(node);

    int balance = get_balance(node);
    // inserted left left
    if (balance > 1 && key < node->left->key)
      return right_rotate(node);

    // inserted right right
    if (balance < -1 && key > node->right->key)
      return left_rotate(node);

    // inserted left right
    if (balance > 1 && key > node->left->key) {
      node->left = left_rotate(node->left);
      return right_rotate(node);
    }

    // inserted right left
    if (balance < -1 && key < node->right->key) {
      node->right = right_rotate(node->right);
      return left_rotate(node);
    }

    return node;
  }

  Node *min_value_node(Node *node) {
    Node *current = node;
    while (current->left != nullptr)
      current = current->left;
    return current;
  }

  Node *delete_node(Node *root, int key) {
    if (root == nullptr)
      return root;

    if (key < root->key)
      root->left = delete_node(root->left, key);
    else if (key > root->key)
      root->right = delete_node(root->right, key);
    else {
      if ((root->left == nullptr) || (root->right == nullptr)) {
        Node *temp = root->left ? root->left : root->right;
        if (temp == nullptr) {
          temp = root;
          root = nullptr;
        } else
          *root = *temp;
        free(temp);
      } else {
        Node *temp = min_value_node(root->right);
        root->key = temp->key;
        root->right = delete_node(root->right, temp->key);
      }
    }
    if (root == nullptr)
      return root;

    root->height = max_height(root) + 1;

    int balance = get_balance(root);

    if (balance > 1 && get_balance(root->left) >= 0)
      return right_rotate(root);

    if (balance > 1 && get_balance(root->left) < 0) {
      root->left = left_rotate(root->left);
      return right_rotate(root);
    }
    if (balance < -1 && get_balance(root->right) <= 0)
      return left_rotate(root);

    if (balance < -1 && get_balance(root->right) > 0) {
      root->right = right_rotate(root->right);
      return left_rotate(root);
    }

    return root;
  }

  Node *root;
};

int main() {
  auto tree = Tree();
  tree.insert(10);
  tree.display();
  tree.insert(20);
  tree.display();
  tree.insert(30);
  tree.display();
  tree.insert(40);
  tree.display();
  tree.insert(50);
  tree.display();
  tree.insert(25);
  tree.display();

  tree.del(30);
  tree.display();
  tree.insert(30);
  tree.display();
  return 0;
}
