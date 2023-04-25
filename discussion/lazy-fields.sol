
struct File
        - (lazy) files(name) => {
                filemap <- load all files
                return filemap[name]
           }
