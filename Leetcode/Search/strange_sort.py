def strangeSort(mapping, nums):
    # Write your code here
    results = []

    for index, n in enumerate(nums):
        string_representation = ""
        for character in n:
            string_representation += str(mapping.index(int(character)))
        results.append((int(string_representation), index, n))

    results.sort()
    to_return = [x[2] for x in results]

    return to_return

if __name__ == "__main__":
    mapping = [2,1,4,8,6,3,0,9,7,5,8]
    nums = ["12","02","4","023","65","83","224","50"]
    strangeSort(mapping,nums)