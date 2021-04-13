#include <cmath>
#include <cstdio>
#include <iostream>

using namespace std;

template <class T> class Tree;

template <class T> class Node {
private:
  T key;
  int height;
  Node *left, *right;

public:
  Node(T key) : key(key), height(1), left(nullptr), right(nullptr) {}

  friend class Tree<T>;
};

enum class Order {
  Pre = 0,
  In = 1,
  Post = 2,
};

template <class T> class Tree {
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
  void display_node(Node<T> *node) {
    auto key = node->key;
    auto leftk = node->left ? node->left->key : -1;
    auto rightk = node->right ? node->right->key : -1;

    cout << " (key=" << key << ",left=" << leftk << ",right=" << rightk << ")";
  }
  void display_pre(Node<T> *node) {
    if (node != nullptr) {
      display_node(node);
      display_pre(node->left);
      display_pre(node->right);
    }
  }
  void display_in(Node<T> *node) {
    if (node != nullptr) {
      display_in(node->left);
      display_node(node);
      display_in(node->right);
    }
  }
  void display_post(Node<T> *node) {
    if (node != nullptr) {
      display_post(node->left);
      display_post(node->right);
      display_node(node);
    }
  }

  inline int height(Node<T> *n) {
    if (n == nullptr)
      return 0;
    return n->height;
  }

  inline int max(int a, int b) { return (a > b) ? a : b; }
  inline int max_height(Node<T> *x) {
    return max(height(x->left), height(x->right));
  }

  Node<T> *right_rotate(Node<T> *y) {
    auto *x = y->left;
    auto *T2 = x->right;

    x->right = y;
    y->left = T2;

    y->height = max_height(y) + 1;
    x->height = max_height(x) + 1;
    return x;
  }

  Node<T> *left_rotate(Node<T> *x) {
    auto *y = x->right;
    auto *T2 = y->left;

    y->left = x;
    x->right = T2;

    x->height = max_height(x) + 1;
    y->height = max_height(y) + 1;
    return y;
  }

  int get_balance(Node<T> *n) {
    if (n == nullptr)
      return 0;
    return height(n->left) - height(n->right);
  }

  Node<T> *insert_with_node(Node<T> *node, T key) {
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

  Node<T> *min_value_node(Node<T> *node) {
    auto current = node;
    while (current->left != nullptr)
      current = current->left;
    return current;
  }

  Node<T> *delete_node(Node<T> *root, T key) {
    if (root == nullptr)
      return root;

    if (key < root->key)
      root->left = delete_node(root->left, key);
    else if (key > root->key)
      root->right = delete_node(root->right, key);
    else {
      if ((root->left == nullptr) || (root->right == nullptr)) {
        auto temp = root->left ? root->left : root->right;
        if (temp == nullptr) {
          temp = root;
          root = nullptr;
        } else
          *root = *temp;
        free(temp);
      } else {
        auto temp = min_value_node(root->right);
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

  Node<T> *root;
};

int main() {
  auto tree = Tree<int>();
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
