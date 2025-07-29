function extractDateViaDateObject(dateTimeStr:string) {
  // 先转换为 Date 对象
  const date = new Date(dateTimeStr);
  
//   // 提取年月日
//   const year = date.getFullYear();
//   const month = String(date.getMonth() + 1).padStart(2, '0');
//   const day = String(date.getDate()).padStart(2, '0');
  
//   return `${year}-${month}-${day}`;
    return date.toLocaleDateString()
}


export {extractDateViaDateObject}