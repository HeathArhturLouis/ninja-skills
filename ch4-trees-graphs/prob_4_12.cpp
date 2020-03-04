/*
    Problem details. We do not know the type of binary tree we have.
    Because of this we make it manually.

    It looks like we just have to go on all possible paths.
    For a path we do not have the start and the end nodes.

    I found 3 types of solution first 2 are very similar. And then the 3rd one might be
    better for speed but has 2^n time.

    Sol1 (implemented): nlogn ? n + logn memory ? 
        Go on each node in preorder traversal.
        Add current node to a path list/vector/array.
        From path[n] to path[0] add each path[i] to a sum check if sum is the one we are looking for.
        If you draw this you see that from the current node we are going upwards and checking
        all partial paths and in the end the path to the node.

    Sol2 :   n log n ? n memory ?  
        Pre order traversal from root node. While going down sum up the values. and check if they are equal to k.
        Move root node on children and repeat.
        this is not saving the path so it might be better for memory usage.

    Sol 3:  this one does not repeat the work since it saves it for later
        Add a list of sums to each node.
        If we had a complete tree
        we could calculate the sums of the partial paths and the end paths when we are constructing the tree.
        Then we just go trough that and check for k in the list of the nodes.
        Bad for memory.

    SOl 3 
    Optimized:
        We are repeteaing work when adding up the sums.
        
        Create a hash table.
        of the form 
        index: 0   1  2  3  4  5  6  7  8
        value: 10  5  1  2 -1 -1  7  1  2
        sum:   10 15 16 18 17 16 23 24 26

        then go down the tree and subtract the indexes above  

        

*/
    



#include <bits/stdc++.h> 
using namespace std; 

//BRUTE FORCE
int paths_total=0;
//utility function to print contents of 
//a vector from index i to it's end 
void printVector(const vector<int>& v, int i) 
{ 
	for (int j=i; j<v.size(); j++) 
		cout << v[j] << " "; 
	cout << endl; 
} 

// binary tree node 
struct Node 
{ 
	int data; 
	Node *left,*right; 
	Node(int x) 
	{ 
		data = x; 
		left = right = NULL; 
	} 
}; 


// traverse pre oreder all nodes
void allPaths(Node *root, vector<int>& path, 
										int k) 
{ 
	// empty node 
	if (!root) 
		return; 

	// add current node to the path 
	path.push_back(root->data); //keep path until current node

	allPaths(root->left, path, k); 

	allPaths(root->right, path, k); 

	int f = 0; 
	for (int j=path.size()-1; j>=0; j--)        //calculate sums by adding more of the nodes on the path     
	{ 
		f += path[j]; 
		// If path sum is k, print the path 
		if (f == k) 
			paths_total++;
	} 
	path.pop_back(); 
} 


int main() 
{ 

    //we do not know if the binary tree is complete so we manualy construct it in order to allow
    // for any kind of tree to be given
	Node *root = new Node(1); 
	root->left = new Node(3); 
	root->left->left = new Node(2); 
	root->left->right = new Node(1); 
	root->left->right->left = new Node(1); 
	root->right = new Node(-1); 
	root->right->left = new Node(4); 
	root->right->left->left = new Node(1); 
	root->right->left->right = new Node(2); 
	root->right->right = new Node(5); 
	root->right->right->right = new Node(2); 

	int k = 5; 
    vector<int> path; 
	allPaths(root,path,k); 
    cout << paths_total;
	return 0; 
} 
