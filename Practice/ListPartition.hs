import Test.QuickCheck
import Data.List.Split

listPartition :: [Int] -> Int -> [[Int]]
listPartition xs n = chunksOf n xs 

case0 = [1, 2, 3, 4, 5, 6, 7]

prop_listPartition = listPartition case0 1 == [[1], [2], [3], [4], [5], [6], [7]]  
                  && listPartition case0 2 == [[1,2], [3,4], [5,6],[7]]
				  && listPartition case0 3 == [[1,2,3],[4,5,6],[7]]
				  && listPartition case0 4 == [[1,2,3,4],[5,6,7]]
				  && listPartition case0 5 == [[1,2,3,4,5],[6,7]]
				  && listPartition case0 6 == [[1,2,3,4,5,6],[7]]
				  && listPartition case0 7 == [[1,2,3,4,5,6,7]]
				  && listPartition case0 8 == [[1,2,3,4,5,6,7]]

main = quickCheck(prop_listPartition)
