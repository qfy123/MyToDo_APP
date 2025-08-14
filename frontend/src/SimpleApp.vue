<template>
  <div class="min-h-screen bg-gray-100 p-8">
    <h1 class="text-3xl font-bold text-blue-600 mb-4">
      调试测试 - 使用TailwindCSS
    </h1>
    
    <div class="bg-white p-6 rounded-lg shadow mb-4">
      <h2 class="text-xl font-semibold mb-2">基础检查</h2>
      <div class="space-y-2">
        <p class="text-green-600">✅ Vue组件已加载</p>
        <p class="text-green-600">✅ TailwindCSS样式已生效</p>
        <p>环境类型: <span class="font-mono text-sm bg-gray-100 px-2 py-1 rounded">{{ envType }}</span></p>
        <p>错误信息: <span class="text-red-600">{{ errorMsg || '无' }}</span></p>
      </div>
    </div>
    
    <div class="bg-white p-6 rounded-lg shadow">
      <h2 class="text-xl font-semibold mb-2">TailwindCSS测试</h2>
      <p class="mb-4">如果你能看到这段文字的样式，说明TailwindCSS正常工作。</p>
      
      <div class="grid grid-cols-3 gap-4 mb-4">
        <div class="bg-red-100 text-red-800 p-3 rounded-lg text-center">红色主题</div>
        <div class="bg-green-100 text-green-800 p-3 rounded-lg text-center">绿色主题</div>
        <div class="bg-blue-100 text-blue-800 p-3 rounded-lg text-center">蓝色主题</div>
      </div>
      
      <button 
        @click="testClick"
        class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors shadow-medium"
      >
        点击测试 ({{ clickCount }})
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';

const envType = ref('未知');
const errorMsg = ref('');
const clickCount = ref(0);

onMounted(() => {
  try {
    if (typeof window !== 'undefined') {
      if ((window as any).__TAURI__) {
        envType.value = 'Tauri桌面环境';
      } else {
        envType.value = '浏览器环境';
      }
    } else {
      envType.value = 'SSR环境';
    }
    console.log('SimpleApp组件已挂载，环境:', envType.value);
  } catch (error) {
    errorMsg.value = (error as Error).message;
    console.error('挂载时出错:', error);
  }
});

const testClick = () => {
  clickCount.value++;
  console.log('按钮被点击，计数:', clickCount.value);
};
</script>