import { createApp } from 'vue'
import SimpleApp from './SimpleApp.vue'
import './style.css'

console.log('main-simple.ts 开始执行');

try {
  const app = createApp(SimpleApp);
  console.log('Vue应用已创建');
  
  app.mount('#app');
  console.log('Vue应用已挂载到#app');
} catch (error) {
  console.error('启动应用时出错:', error);
  document.body.innerHTML = `<div style="padding: 20px; background: red; color: white;">启动错误: ${(error as Error).message}</div>`;
}