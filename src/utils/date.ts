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

/**
 * 格式化日期字符串为简洁易读的日期形式（不包含时分秒）
 * @param dateString 日期字符串，如 "2025-10-03 02:00:46 +08:00"
 * @returns 格式化后的日期字符串，如 "2025-10-03"
 */
function formatDateOnly(dateString: string): string {
  try {
    const date = new Date(dateString);
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
  } catch (error) {
    console.error('日期格式化错误:', error);
    return dateString; // 如果解析失败，返回原字符串
  }
}


function formatTimeOnly(dateString: string): string {
  try {
    const date = new Date(dateString);
    const hour = String(date.getHours()).padStart(2, '0');
    const minute = String(date.getMinutes()).padStart(2, '0');
    const second = String(date.getSeconds()).padStart(2, '0');
    return `${hour}:${minute}:${second}`;
  } catch (error) {
    console.error('时间格式化错误:', error);
    return dateString; // 如果解析失败，返回原字符串
  }
}

function formatHourAndMinute(dateString: string): string {
  try {
    const date = new Date(dateString);
    const hour = String(date.getHours()).padStart(2, '0');
    const minute = String(date.getMinutes()).padStart(2, '0');
    return `${hour}:${minute}`;
  } catch (error) {
    console.error('时间格式化错误:', error);
    return dateString; // 如果解析失败，返回原字符串
  }
}



export {extractDateViaDateObject,extractTimeStampSecond,getTodayRange,formatDatetime,formatDateOnly,formatTimeOnly,formatHourAndMinute}



