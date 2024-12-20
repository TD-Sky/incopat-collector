// ==UserScript==
// @name        patent
// @namespace   Violentmonkey Scripts
// @match       https://www.incopat.com/downloadTemp/toDownloadPage*
// @grant       none
// @version     1.0
// @author      -
// @description 2024/11/23 14:34:19
// @run-at document-end
// ==/UserScript==

const INTERVAL_SECS = 3; // 下载间隔时间（单位：秒）
const sleep = (delay) => new Promise((resolve) => setTimeout(resolve, delay));

(async function () {
  // 清空所有字段
  let checkAllElement = document.querySelector(
    '[name="checkAllDownloadField"]',
  );
  checkAllElement.click();
  checkAllElement.click();

  // 需要导出的字段
  let checkBoxes = ["申请人", "申请号", "专利类型", "公开(公告)日"];

  checkBoxes.forEach(function (name) {
    let checkBox = document.querySelector(`[fieldName="${name}"]`);
    checkBox.click();
  });

  // 获取条目总数
  let items = Number(
    document.querySelector("#load_count_label").textContent.match(/\d+/)[0],
  );
  let download = document.querySelector('[value="下载"]');

  // 逐批次下载，在下载前等待一定时间
  for (let i = 1; i <= items; i += 50) {
    let end = Math.min(i + 49, items);
    document.querySelector("#rangeValue").value = `${i}-${end}`;
    await sleep(INTERVAL_SECS * 1000);
    download.click();
  }
})();
