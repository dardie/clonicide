#Overview of implementation

Brief, simplified explanation of my algorithm:

1. For every file on the filesystem, calculate a hash of its contents.

2. Create a table `file_idx` of all files whose contents match a particular hashvalue.

3. From `file_idx`, create a table `matched_pair_idx` for each pair of files that match.

4. More specifically: if `folder1/filename1` and `folder2/filename2` have the same hashvalue,
and `Ord(folder1)` > `Ord(folder2)`,
there should be table entry with key `<folder1, folder2>` containing a list of `MatchedFilePairs` (paired files with the same hash value) as the value.
MatchedFilePairs look like `<hashvalue, filename1, filename2>`.

5. Once all files with matching hash values have been added to the `matched_pair_idx`,
find `MatchedDirPairs` (ie. `<folder1, folder2>`) where the number of matched files is equal to (or approaching)
the number of files in folder1 and folder2.

6. Bob is, as they say, your Uncle. You now have a list of pairs of folders that have files with the same contents.
Even if, for example, every filename has been modified in some way, this algorithm will not be fooled.