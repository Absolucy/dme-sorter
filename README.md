# dme-sorter

simple thingy I made that automatically sorts and cleans up the includes of dme files.  
specifically, it does these in order:
1. remove any include statements to files that don't exist
2. sorts include statements (should be same sorting logic as dreammaker)
3. removes any duplicates
4. re-saves the .dme file with the changes

the `diff` feature (enabled by default) outputs a diff of the dme to stdout whenever it's run.

also it automatically skips over merge conflict markers (they will not be present in the output, even if they were in the input) so you can literally use this on a merge conflicted dme to resolve the conflicts, prolly. (haven't tested this but it's just an automatic way of how I solve dme conflicts anyways lol)

should be relatively simple to use:  
`dme-sorter whatever.dme` on the command line.  
if you want a separate output file instead of replacing the original, you can do `dme-sorter whatever.dme whatever-sorted.dme` instead

prebuilt binaries available in github actions runs: https://github.com/Absolucy/dme-sorter/actions

dme-sorter is licensed under the 0BSD license, see [LICENSE.md]
