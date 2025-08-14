<template>
  <nav class="w-72 bg-white/70 dark:bg-gray-900/70 backdrop-blur-sm rounded-2xl shadow-soft dark:shadow-soft-dark border border-white/50 dark:border-gray-700/50 p-6 h-fit sticky top-24 transition-colors duration-300">
    <div class="mb-8">
      <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-6 flex items-center space-x-2 transition-colors duration-300">
        <span class="w-6 h-6 bg-gradient-to-br from-primary-500 to-primary-600 rounded-lg flex items-center justify-center">
          <svg class="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
          </svg>
        </span>
        <span>任务视图</span>
      </h2>
      
      <ul class="space-y-3">
        <li>
          <button 
            @click="setActiveView('today')"
            :class="[
              'w-full flex items-center px-4 py-3 text-left rounded-xl transition-all duration-200 group',
              activeView === 'today' 
                ? 'bg-gradient-to-r from-yellow-50 to-orange-50 dark:from-yellow-900/30 dark:to-orange-900/30 text-yellow-700 dark:text-yellow-300 shadow-soft dark:shadow-soft-dark border border-yellow-200/50 dark:border-yellow-700/50' 
                : 'text-gray-700 dark:text-gray-300 hover:bg-gray-50/80 dark:hover:bg-gray-700/50 hover:shadow-soft dark:hover:shadow-soft-dark'
            ]"
          >
            <span class="mr-3 text-lg">📅</span>
            <span class="flex-1 font-medium">今日任务</span>
            <span v-if="todayCount > 0" :class="[
              'text-xs px-2 py-1 rounded-lg font-medium transition-colors duration-300',
              activeView === 'today' 
                ? 'bg-yellow-200/80 dark:bg-yellow-800/60 text-yellow-800 dark:text-yellow-200' 
                : 'bg-primary-100 dark:bg-primary-900/50 text-primary-700 dark:text-primary-300 group-hover:bg-primary-200 dark:group-hover:bg-primary-800/60'
            ]">
              {{ todayCount }}
            </span>
          </button>
        </li>
        
        <li>
          <button 
            @click="setActiveView('all')"
            :class="[
              'w-full flex items-center px-4 py-3 text-left rounded-xl transition-all duration-200 group',
              activeView === 'all' 
                ? 'bg-gradient-to-r from-blue-50 to-blue-50 dark:from-blue-900/30 dark:to-blue-900/30 text-blue-700 dark:text-blue-300 shadow-soft dark:shadow-soft-dark border border-blue-200/50 dark:border-blue-700/50' 
                : 'text-gray-700 dark:text-gray-300 hover:bg-gray-50/80 dark:hover:bg-gray-700/50 hover:shadow-soft dark:hover:shadow-soft-dark'
            ]"
          >
            <span class="mr-3 text-lg">📋</span>
            <span class="flex-1 font-medium">所有任务</span>
            <span v-if="allCount > 0" :class="[
              'text-xs px-2 py-1 rounded-lg font-medium transition-colors duration-300',
              activeView === 'all' 
                ? 'bg-blue-200/80 dark:bg-blue-800/60 text-blue-800 dark:text-blue-200' 
                : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 group-hover:bg-gray-300 dark:group-hover:bg-gray-600'
            ]">
              {{ allCount }}
            </span>
          </button>
        </li>
        
        <li>
          <button 
            @click="setActiveView('completed')"
            :class="[
              'w-full flex items-center px-4 py-3 text-left rounded-xl transition-all duration-200 group',
              activeView === 'completed' 
                ? 'bg-gradient-to-r from-green-50 to-green-50 dark:from-green-900/30 dark:to-green-900/30 text-green-700 dark:text-green-300 shadow-soft dark:shadow-soft-dark border border-green-200/50 dark:border-green-700/50' 
                : 'text-gray-700 dark:text-gray-300 hover:bg-gray-50/80 dark:hover:bg-gray-700/50 hover:shadow-soft dark:hover:shadow-soft-dark'
            ]"
          >
            <span class="mr-3 text-lg">✅</span>
            <span class="flex-1 font-medium">已完成</span>
            <span v-if="completedCount > 0" :class="[
              'text-xs px-2 py-1 rounded-lg font-medium transition-colors duration-300',
              activeView === 'completed' 
                ? 'bg-green-200/80 dark:bg-green-800/60 text-green-800 dark:text-green-200' 
                : 'bg-green-100 dark:bg-green-900/50 text-green-700 dark:text-green-300 group-hover:bg-green-200 dark:group-hover:bg-green-800/60'
            ]">
              {{ completedCount }}
            </span>
          </button>
        </li>
      </ul>
    </div>
    
    <div class="border-t border-gray-200/50 dark:border-gray-700/50 pt-6 mb-8 transition-colors duration-300">
      <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-4 flex items-center space-x-2 transition-colors duration-300">
        <span class="w-5 h-5 bg-gradient-to-br from-gray-400 to-gray-500 dark:from-gray-500 dark:to-gray-600 rounded-md flex items-center justify-center">
          <svg class="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
        </span>
        <span>数据管理</span>
      </h3>
      
      <ul class="space-y-2">
        <li>
          <button 
            @click="setActiveView('settings')"
            :class="[
              'w-full flex items-center px-4 py-3 text-left rounded-xl transition-all duration-200',
              activeView === 'settings' 
                ? 'bg-gradient-to-r from-gray-50 to-gray-50 dark:from-gray-800/50 dark:to-gray-800/50 text-gray-700 dark:text-gray-300 shadow-soft dark:shadow-soft-dark border border-gray-200/50 dark:border-gray-700/50' 
                : 'text-gray-700 dark:text-gray-300 hover:bg-gray-50/80 dark:hover:bg-gray-700/50 hover:shadow-soft dark:hover:shadow-soft-dark'
            ]"
          >
            <span class="mr-3 text-lg">⚙️</span>
            <span class="font-medium">设置</span>
          </button>
        </li>
      </ul>
    </div>
    
    <!-- 快速添加任务按钮 -->
    <div class="border-t border-gray-200/50 dark:border-gray-700/50 pt-6 transition-colors duration-300">
      <button 
        @click="$emit('showAddTask')"
        class="w-full bg-gradient-to-r from-primary-500 to-primary-600 hover:from-primary-600 hover:to-primary-700 text-white font-medium py-3 px-4 rounded-xl transition-all duration-200 shadow-medium hover:shadow-lg transform hover:-translate-y-0.5 flex items-center justify-center space-x-2"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
        </svg>
        <span>添加新任务</span>
      </button>
    </div>
    
    <!-- 应用信息 -->
    <div class="mt-8 p-4 bg-gradient-to-br from-gray-50/50 to-gray-100/50 dark:from-gray-800/30 dark:to-gray-700/30 rounded-xl border border-gray-200/30 dark:border-gray-700/30 transition-colors duration-300">
      <div class="text-center">
        <div class="w-12 h-12 bg-gradient-to-br from-primary-500 to-primary-600 rounded-xl flex items-center justify-center mx-auto mb-3">
          <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" />
          </svg>
        </div>
        <h4 class="font-medium text-gray-900 dark:text-gray-100 text-sm transition-colors duration-300">MyTodo</h4>
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-1 transition-colors duration-300">v0.1.0</p>
        <div class="mt-3 text-xs text-gray-400 dark:text-gray-500 transition-colors duration-300">
          <p>Rust + Tauri + Vue3</p>
          <p class="mt-1">🔒 本地存储，保护隐私</p>
        </div>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
interface Props {
  activeView: string;
  todayCount?: number;
  allCount?: number;
  completedCount?: number;
}

interface Emits {
  (e: 'update:activeView', value: string): void;
  (e: 'showAddTask'): void;
}

const props = withDefaults(defineProps<Props>(), {
  todayCount: 0,
  allCount: 0,
  completedCount: 0
});

const emit = defineEmits<Emits>();

const setActiveView = (view: string) => {
  emit('update:activeView', view);
};
</script>