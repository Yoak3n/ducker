const simplifyPath = (path:string) => {
    if (!path) return '';
    const separator = path.includes('\\') ? '\\' : '/';
    const parts = path.split(separator);
    const filename = parts[parts.length - 1];
    if (parts.length > 4) {
      return parts[0] + separator + '...' + separator + parts[parts.length - 2] + separator + filename;
    }
    
    return path;
};


export {simplifyPath}