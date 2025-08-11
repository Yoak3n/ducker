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
function extractTimeStampSecond(dateTimeStr:string) {
    const date = new Date(dateTimeStr);
    return date.getTime()/1000
}

function getTodayRange(): { start: number; end: number } {
  const now = new Date();
  now.setHours(0, 0, 0, 0);
  const startOfDay = Math.floor(now.getTime() / 1000);
  
  now.setHours(23, 59, 59, 999);
  const endOfDay = Math.floor(now.getTime() / 1000);
  
  return { start: startOfDay, end: endOfDay };
}

function formatDatetime(date: Date) {
  const year = date.getFullYear()
  const month = ('0' + (date.getMonth() + 1)).slice(-2)
  const day = ('0' + date.getDate()).slice(-2)
  const hour = ('0' + date.getHours()).slice(-2)
  const minute = ('0' + date.getMinutes()).slice(-2)
  const second = ('0' + date.getSeconds()).slice(-2)
    return `${year}-${month}-${day} ${hour}:${minute}:${second}`


}

export {extractDateViaDateObject,extractTimeStampSecond,getTodayRange,formatDatetime}



