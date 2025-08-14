<template>
  <div class="min-h-screen bg-gray-100 p-8">
    <div class="max-w-4xl mx-auto">
      <h1 class="text-3xl font-bold text-blue-600 mb-8">MyTodo - 个人待办事项</h1>
      
      <div class="bg-white rounded-lg shadow p-6 mb-6">
        <h2 class="text-xl font-semibold mb-4">应用状态检查</h2>
        
        <div class="space-y-3">
          <div class="flex items-center space-x-3">
            <div class="w-3 h-3 bg-green-500 rounded-full"></div>
            <span>Vue3 已加载</span>
          </div>
          
          <div class="flex items-center space-x-3">
            <div class="w-3 h-3 bg-green-500 rounded-full"></div>
            <span>TailwindCSS 样式已生效</span>
          </div>
          
          <div class="flex items-center space-x-3">
            <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
            <span>Tauri API 连接状态: {{ tauriStatus }}</span>
          </div>
        </div>
        
        <button 
          @click="testTauriAPI"
          class="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
        >
          测试 Tauri API 连接
        </button>
      </div>
      
      <div class="bg-white rounded-lg shadow p-6">
        <h2 class="text-xl font-semibold mb-4">简单任务列表测试</h2>
        
        <div class="space-y-2">
          <div class="flex items-center space-x-3 p-3 border rounded">
            <input type="checkbox" class="rounded">
            <span>测试任务 1 - 检查应用是否正常运行</span>
            <span class="ml-auto text-sm text-gray-500">★★☆ 中</span>
          </div>
          
          <div class="flex items-center space-x-3 p-3 border rounded">
            <input type="checkbox" class="rounded">
            <span>测试任务 2 - 验证界面显示正常</span>
            <span class="ml-auto text-sm text-gray-500">★☆☆ 低</span>
          </div>
        </div>
        
        <button class="mt-4 w-full border-2 border-dashed border-gray-300 rounded py-3 text-gray-500 hover:border-gray-400">
          + 添加测试任务
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';

const tauriStatus = ref('检测中...');

onMounted(() => {
  // 检查是否在 Tauri 环境中
  if (typeof window !== 'undefined' && (window as any).__TAURI__) {
    tauriStatus.value = '已连接';
  } else {
    tauriStatus.value = '未连接 (浏览器模式)';
  }
});

const testTauriAPI = async () => {
  try {
    if (typeof window !== 'undefined' && (window as any).__TAURI__) {
      // 在 Tauri 环境中测试 API
      const { invoke } = await import('@tauri-apps/api/core');
      const result = await invoke('get_all_tasks');
      alert('Tauri API 测试成功!');
      console.log('API 响应:', result);
    } else {
      alert('当前运行在浏览器模式，Tauri API 不可用');
    }
  } catch (error) {
    console.error('Tauri API 测试失败:', error);
    alert('Tauri API 测试失败: ' + (error as Error).message);
  }
};
</script>