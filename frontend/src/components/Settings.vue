<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm dark:shadow-soft-dark p-6 transition-colors duration-300">
    <h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-6 transition-colors duration-300">设置</h2>
    
    <!-- 应用行为设置 -->
    <div class="space-y-6">
      <!-- 关闭行为设置 -->
      <div class="border-b border-gray-200 dark:border-gray-700 pb-6 transition-colors duration-300">
        <h3 class="text-lg font-medium text-gray-900 dark:text-gray-100 mb-4 transition-colors duration-300">应用行为</h3>
        
        <div class="space-y-4">
          <!-- 关闭程序时的行为 -->
          <div class="flex items-center justify-between">
            <div>
              <h4 class="text-sm font-medium text-gray-900 dark:text-gray-100 transition-colors duration-300">关闭程序时</h4>
              <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">选择点击关闭按钮时的行为</p>
            </div>
            <select 
              v-model="settings.close_behavior"
              @change="updateSettings"
              class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 text-sm focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-300"
            >
              <option value="ask">每次询问</option>
              <option value="exit">直接退出</option>
              <option value="minimize">最小化到托盘</option>
            </select>
          </div>
          
          <!-- 启动时行为 -->
          <div class="flex items-center justify-between">
            <div>
              <h4 class="text-sm font-medium text-gray-900 dark:text-gray-100 transition-colors duration-300">启动时行为</h4>
              <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">应用启动时的默认行为</p>
            </div>
            <select 
              v-model="settings.startup_behavior"
              @change="updateSettings"
              class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 text-sm focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-300"
            >
              <option value="normal">正常启动</option>
              <option value="minimized">最小化到托盘</option>
            </select>
          </div>
        </div>
      </div>
      
      <!-- 通知设置 -->
      <div class="border-b border-gray-200 dark:border-gray-700 pb-6 transition-colors duration-300">
        <h3 class="text-lg font-medium text-gray-900 dark:text-gray-100 mb-4 transition-colors duration-300">通知设置</h3>
        
        <div class="space-y-4">
          <!-- 启用通知 -->
          <div class="flex items-center justify-between">
            <div>
              <h4 class="text-sm font-medium text-gray-900 dark:text-gray-100 transition-colors duration-300">启用通知</h4>
              <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">接收任务截止时间提醒</p>
            </div>
            <input 
              type="checkbox" 
              v-model="settings.notifications_enabled"
              @change="updateSettings"
              class="h-5 w-5 text-primary-600 border-gray-300 dark:border-gray-600 rounded focus:ring-primary-500"
            />
          </div>
          
          <!-- 提前通知时间 -->
          <div class="flex items-center justify-between" v-if="settings.notifications_enabled">
            <div>
              <h4 class="text-sm font-medium text-gray-900 dark:text-gray-100 transition-colors duration-300">提前通知时间</h4>
              <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">截止时间前多久发送通知</p>
            </div>
            <div class="flex items-center space-x-2">
              <input 
                type="number" 
                v-model.number="settings.notification_time_before"
                @change="updateSettings"
                min="1"
                max="1440"
                class="w-20 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 text-sm focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-300"
              />
              <span class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">分钟</span>
            </div>
          </div>
        </div>
      </div>
    
      <!-- 数据管理部分 -->
      <div class="border-b border-gray-200 dark:border-gray-700 pb-6 transition-colors duration-300">
        <h3 class="text-lg font-medium text-gray-900 dark:text-gray-100 mb-4 transition-colors duration-300">数据备份</h3>
        
        <div class="space-y-4">
          <!-- 导出数据 -->
          <div class="flex items-center justify-between">
            <div>
              <h4 class="text-sm font-medium text-gray-900 dark:text-gray-100 transition-colors duration-300">导出任务数据</h4>
              <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">将所有任务数据导出为 JSON 文件</p>
            </div>
            <button 
              @click="exportTasks"
              :disabled="isExporting"
              class="px-4 py-2 bg-blue-500 text-white text-sm font-medium rounded-md hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-300"
            >
              {{ isExporting ? '导出中...' : '导出数据' }}
            </button>
          </div>
          
          <!-- 导入数据 -->
          <div class="flex items-center justify-between">
            <div>
              <h4 class="text-sm font-medium text-gray-900 dark:text-gray-100 transition-colors duration-300">导入任务数据</h4>
              <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">从 JSON 文件导入任务数据</p>
            </div>
            <div class="space-x-2">
              <input 
                ref="fileInput"
                type="file" 
                accept=".json"
                @change="handleFileSelect"
                class="hidden"
              />
              <button 
                @click="triggerFileSelect"
                :disabled="isImporting"
                class="px-4 py-2 bg-green-500 text-white text-sm font-medium rounded-md hover:bg-green-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-300"
              >
                {{ isImporting ? '导入中...' : '选择文件' }}
              </button>
            </div>
          </div>
          
          <!-- 导入进度显示 -->
          <div v-if="importResult" class="mt-4 p-3 rounded-md transition-colors duration-300" :class="[
            importResult.success 
              ? 'bg-green-50 dark:bg-green-900/30 border border-green-200 dark:border-green-700/50' 
              : 'bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-700/50'
          ]">
            <p :class="[
              'text-sm font-medium transition-colors duration-300',
              importResult.success ? 'text-green-800 dark:text-green-200' : 'text-red-800 dark:text-red-200'
            ]">
              {{ importResult.message }}
            </p>
          </div>
        </div>
      </div>
      
      <!-- 数据清理 -->
      <div class="border-b border-gray-200 dark:border-gray-700 pb-6 transition-colors duration-300">
        <h3 class="text-lg font-medium text-gray-900 dark:text-gray-100 mb-4 transition-colors duration-300">数据清理</h3>
        
        <div class="space-y-4">
          <!-- 清空所有任务 -->
          <div class="flex items-center justify-between">
            <div>
              <h4 class="text-sm font-medium text-gray-900 dark:text-gray-100 transition-colors duration-300">清空所有任务</h4>
              <p class="text-sm text-red-600 dark:text-red-400 transition-colors duration-300">⚠️ 此操作将删除所有任务数据，无法恢复</p>
            </div>
            <button 
              @click="confirmClearAll"
              :disabled="isClearing"
              class="px-4 py-2 bg-red-500 text-white text-sm font-medium rounded-md hover:bg-red-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-300"
            >
              {{ isClearing ? '清空中...' : '清空数据' }}
            </button>
          </div>
        </div>
      </div>
      
      <!-- 应用信息 -->
      <div>
        <h3 class="text-lg font-medium text-gray-900 dark:text-gray-100 mb-4 transition-colors duration-300">应用信息</h3>
        
        <div class="space-y-3 text-sm">
          <div class="flex justify-between">
            <span class="text-gray-500 dark:text-gray-400 transition-colors duration-300">应用名称:</span>
            <span class="text-gray-900 dark:text-gray-100 transition-colors duration-300">MyTodo 个人待办事项</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-500 dark:text-gray-400 transition-colors duration-300">版本:</span>
            <span class="text-gray-900 dark:text-gray-100 transition-colors duration-300">v0.1.0</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-500 dark:text-gray-400 transition-colors duration-300">数据存储位置:</span>
            <span class="text-gray-900 dark:text-gray-100 font-mono text-xs transition-colors duration-300">%UserProfile%\Documents\TodoAppData\</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-500 dark:text-gray-400 transition-colors duration-300">技术栈:</span>
            <span class="text-gray-900 dark:text-gray-100 transition-colors duration-300">Rust + Tauri + Vue3</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { TaskAPI } from '../utils/taskAPI';

interface Emits {
  (e: 'dataChanged'): void;
}

interface AppSettings {
  close_behavior: 'exit' | 'minimize' | 'ask';
  notifications_enabled: boolean;
  notification_time_before: number;
  startup_behavior: 'normal' | 'minimized';
  theme: string;
}

const emit = defineEmits<Emits>();

const fileInput = ref<HTMLInputElement>();
const isExporting = ref(false);
const isImporting = ref(false);
const isClearing = ref(false);
const importResult = ref<{ success: boolean; message: string } | null>(null);

const settings = ref<AppSettings>({
  close_behavior: 'ask',
  notifications_enabled: true,
  notification_time_before: 15,
  startup_behavior: 'normal',
  theme: 'auto'
});

// 加载设置
onMounted(async () => {
  try {
    const appSettings = await TaskAPI.getAppSettings();
    settings.value = appSettings;
  } catch (error) {
    console.error('加载设置失败:', error);
  }
});

// 更新设置
const updateSettings = async () => {
  try {
    await TaskAPI.updateAppSettings(settings.value);
    
    // 显示更新成功消息
    importResult.value = {
      success: true,
      message: '设置已保存！'
    };
    
    // 3秒后清除消息
    setTimeout(() => {
      importResult.value = null;
    }, 3000);
    
  } catch (error) {
    console.error('更新设置失败:', error);
    importResult.value = {
      success: false,
      message: '保存设置失败: ' + (error as Error).message
    };
  }
};

// 导出任务数据
const exportTasks = async () => {
  isExporting.value = true;
  
  try {
    const jsonData = await TaskAPI.exportTasksToJson();
    
    // 创建下载链接
    const blob = new Blob([jsonData], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `mytodo-backup-${new Date().toISOString().split('T')[0]}.json`;
    
    // 触发下载
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
    
    // 显示成功消息
    importResult.value = {
      success: true,
      message: '任务数据导出成功！'
    };
    
    // 3秒后清除消息
    setTimeout(() => {
      importResult.value = null;
    }, 3000);
    
  } catch (error) {
    console.error('导出失败:', error);
    importResult.value = {
      success: false,
      message: '导出失败: ' + (error as Error).message
    };
  } finally {
    isExporting.value = false;
  }
};

// 触发文件选择
const triggerFileSelect = () => {
  fileInput.value?.click();
};

// 处理文件选择
const handleFileSelect = async (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;
  
  // 验证文件类型
  if (!file.name.toLowerCase().endsWith('.json')) {
    importResult.value = {
      success: false,
      message: '请选择 JSON 格式的文件'
    };
    return;
  }
  
  isImporting.value = true;
  importResult.value = null;
  
  try {
    // 读取文件内容
    const fileContent = await file.text();
    
    // 验证 JSON 格式
    let parsedData;
    try {
      parsedData = JSON.parse(fileContent);
    } catch {
      throw new Error('文件格式错误，请确保是有效的 JSON 文件');
    }
    
    // 导入数据
    const importedCount = await TaskAPI.importTasksFromJson(fileContent);
    
    importResult.value = {
      success: true,
      message: `成功导入 ${importedCount} 个任务！`
    };
    
    // 通知父组件数据已更改
    emit('dataChanged');
    
  } catch (error) {
    console.error('导入失败:', error);
    importResult.value = {
      success: false,
      message: '导入失败: ' + (error as Error).message
    };
  } finally {
    isImporting.value = false;
    // 清空文件输入
    if (fileInput.value) {
      fileInput.value.value = '';
    }
  }
};

// 确认清空所有数据
const confirmClearAll = () => {
  const confirmed = confirm(
    '⚠️ 警告：此操作将永久删除所有任务数据，无法恢复！\n\n' +
    '建议在清空前先导出数据进行备份。\n\n' +
    '确定要继续吗？'
  );
  
  if (confirmed) {
    clearAllTasks();
  }
};

// 清空所有任务
const clearAllTasks = async () => {
  isClearing.value = true;
  
  try {
    await TaskAPI.clearAllTasks();
    
    importResult.value = {
      success: true,
      message: '所有任务数据已清空！'
    };
    
    // 通知父组件数据已更改
    emit('dataChanged');
    
  } catch (error) {
    console.error('清空失败:', error);
    importResult.value = {
      success: false,
      message: '清空失败: ' + (error as Error).message
    };
  } finally {
    isClearing.value = false;
  }
};
</script>