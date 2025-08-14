<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800 transition-colors duration-300">
    <!-- 顶部导航栏 -->
    <header class="bg-white/80 dark:bg-gray-900/80 backdrop-blur-md shadow-soft dark:shadow-soft-dark border-b border-gray-200/50 dark:border-gray-700/50 sticky top-0 z-50 transition-colors duration-300">
      <div class="max-w-7xl mx-auto px-6 py-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-3">
            <div class="w-8 h-8 bg-gradient-to-br from-primary-500 to-primary-600 rounded-xl flex items-center justify-center">
              <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" />
              </svg>
            </div>
            <h1 class="text-2xl font-semibold text-gray-900 dark:text-gray-100 transition-colors duration-300">MyTodo</h1>
            <span class="text-sm text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded-lg transition-colors duration-300">个人待办事项</span>
          </div>
          
          <!-- 主题切换按钮 -->
          <ThemeToggle />
        </div>
      </div>
    </header>
    
    <main class="max-w-7xl mx-auto px-6 py-8">
      <div class="flex gap-8">
        <!-- 左侧导航栏 -->
        <Navigation 
          v-model:activeView="activeView"
          :todayCount="todayTasks.length"
          :allCount="allTasks.filter(t => !t.is_completed).length"
          :completedCount="completedTasks.length"
          @showAddTask="showTaskForm()"
        />
        
        <!-- 主内容区 -->
        <div class="flex-1 min-w-0">
          <!-- 今日任务视图 -->
          <div v-if="activeView === 'today'" class="bg-white/70 dark:bg-gray-900/70 backdrop-blur-sm rounded-2xl shadow-soft dark:shadow-soft-dark border border-white/50 dark:border-gray-700/50 p-8 transition-colors duration-300">
            <div class="flex items-center justify-between mb-8">
              <div class="flex items-center space-x-3">
                <div class="w-10 h-10 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-xl flex items-center justify-center">
                  <span class="text-xl">📅</span>
                </div>
                <div>
                  <h2 class="text-2xl font-semibold text-gray-900 dark:text-gray-100 transition-colors duration-300">今日任务</h2>
                  <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">{{ new Date().toLocaleDateString('zh-CN', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' }) }}</p>
                </div>
              </div>
              <button 
                @click="showTaskForm()"
                class="px-6 py-3 bg-gradient-to-r from-primary-500 to-primary-600 text-white font-medium rounded-xl hover:from-primary-600 hover:to-primary-700 transition-all duration-200 shadow-medium hover:shadow-lg transform hover:-translate-y-0.5"
              >
                <span class="flex items-center space-x-2">
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                  </svg>
                  <span>添加任务</span>
                </span>
              </button>
            </div>
            
            <!-- 搜索过滤组件 -->
            <SearchFilter 
              :tasks="todayTasks"
              @filter="handleTodayFilter"
            />
            
            <div v-if="loading" class="text-center py-16">
              <div class="inline-flex items-center space-x-3">
                <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-primary-500"></div>
                <span class="text-gray-500 dark:text-gray-400 transition-colors duration-300">加载中...</span>
              </div>
            </div>
            
            <div v-else-if="filteredTodayTasks.length === 0 && todayTasks.length === 0" class="text-center py-16">
              <div class="w-20 h-20 bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-800 dark:to-gray-700 rounded-2xl flex items-center justify-center mx-auto mb-6 transition-colors duration-300">
                <span class="text-3xl">🎉</span>
              </div>
              <div class="text-gray-600 dark:text-gray-300 mb-6 transition-colors duration-300">
                <h3 class="text-lg font-medium mb-2">今天还没有任务</h3>
                <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">开始添加一些任务来组织你的一天吧</p>
              </div>
              <button 
                @click="showTaskForm()"
                class="px-6 py-3 bg-gradient-to-r from-primary-500 to-primary-600 text-white font-medium rounded-xl hover:from-primary-600 hover:to-primary-700 transition-all duration-200 transform hover:-translate-y-0.5"
              >
                创建第一个任务
              </button>
            </div>

            <div v-else-if="filteredTodayTasks.length === 0" class="text-center py-16">
              <div class="w-20 h-20 bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-800 dark:to-gray-700 rounded-2xl flex items-center justify-center mx-auto mb-6 transition-colors duration-300">
                <span class="text-3xl">🔍</span>
              </div>
              <div class="text-gray-600 dark:text-gray-300 transition-colors duration-300">
                <h3 class="text-lg font-medium mb-2">没有找到匹配的任务</h3>
                <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">尝试调整搜索条件或筛选器</p>
              </div>
            </div>
            
            <div v-else class="space-y-4">
              <TaskCard 
                v-for="(task, index) in filteredTodayTasks"
                :key="task.id"
                :task="task"
                :index="index"
                @toggle="toggleTaskCompletion"
                @edit="showTaskForm"
                @delete="deleteTask"
              />
            </div>
          </div>
          
          <!-- 所有任务视图 -->
          <div v-else-if="activeView === 'all'" class="bg-white/70 dark:bg-gray-900/70 backdrop-blur-sm rounded-2xl shadow-soft dark:shadow-soft-dark border border-white/50 dark:border-gray-700/50 p-8 transition-colors duration-300">
            <div class="flex items-center justify-between mb-8">
              <div class="flex items-center space-x-3">
                <div class="w-10 h-10 bg-gradient-to-br from-blue-400 to-blue-600 rounded-xl flex items-center justify-center">
                  <span class="text-xl">📋</span>
                </div>
                <div>
                  <h2 class="text-2xl font-semibold text-gray-900 dark:text-gray-100 transition-colors duration-300">所有任务</h2>
                  <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">按优先级和截止日期排序</p>
                </div>
              </div>
              <button 
                @click="showTaskForm()"
                class="px-6 py-3 bg-gradient-to-r from-primary-500 to-primary-600 text-white font-medium rounded-xl hover:from-primary-600 hover:to-primary-700 transition-all duration-200 shadow-medium hover:shadow-lg transform hover:-translate-y-0.5"
              >
                <span class="flex items-center space-x-2">
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                  </svg>
                  <span>添加任务</span>
                </span>
              </button>
            </div>
            
            <!-- 搜索过滤组件 -->
            <SearchFilter 
              :tasks="activeTasks"
              @filter="handleAllFilter"
            />
            
            <div v-if="loading" class="text-center py-16">
              <div class="inline-flex items-center space-x-3">
                <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-primary-500"></div>
                <span class="text-gray-500 dark:text-gray-400 transition-colors duration-300">加载中...</span>
              </div>
            </div>
            
            <div v-else-if="filteredAllTasks.length === 0 && activeTasks.length === 0" class="text-center py-16">
              <div class="w-20 h-20 bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-800 dark:to-gray-700 rounded-2xl flex items-center justify-center mx-auto mb-6 transition-colors duration-300">
                <span class="text-3xl">📝</span>
              </div>
              <div class="text-gray-600 dark:text-gray-300 mb-6 transition-colors duration-300">
                <h3 class="text-lg font-medium mb-2">暂无未完成任务</h3>
                <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">所有任务都已完成，或者开始创建新任务</p>
              </div>
              <button 
                @click="showTaskForm()"
                class="px-6 py-3 bg-gradient-to-r from-primary-500 to-primary-600 text-white font-medium rounded-xl hover:from-primary-600 hover:to-primary-700 transition-all duration-200 transform hover:-translate-y-0.5"
              >
                创建新任务
              </button>
            </div>

            <div v-else-if="filteredAllTasks.length === 0" class="text-center py-16">
              <div class="w-20 h-20 bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-800 dark:to-gray-700 rounded-2xl flex items-center justify-center mx-auto mb-6 transition-colors duration-300">
                <span class="text-3xl">🔍</span>
              </div>
              <div class="text-gray-600 dark:text-gray-300 transition-colors duration-300">
                <h3 class="text-lg font-medium mb-2">没有找到匹配的任务</h3>
                <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">尝试调整搜索条件或筛选器</p>
              </div>
            </div>
            
            <div v-else class="space-y-4">
              <TaskCard 
                v-for="(task, index) in filteredAllTasks"
                :key="task.id"
                :task="task"
                :index="index"
                @toggle="toggleTaskCompletion"
                @edit="showTaskForm"
                @delete="deleteTask"
              />
            </div>
          </div>
          
          <!-- 已完成任务视图 -->
          <div v-else-if="activeView === 'completed'" class="bg-white/70 dark:bg-gray-900/70 backdrop-blur-sm rounded-2xl shadow-soft dark:shadow-soft-dark border border-white/50 dark:border-gray-700/50 p-8 transition-colors duration-300">
            <div class="flex items-center justify-between mb-8">
              <div class="flex items-center space-x-3">
                <div class="w-10 h-10 bg-gradient-to-br from-green-400 to-green-600 rounded-xl flex items-center justify-center">
                  <span class="text-xl">✅</span>
                </div>
                <div>
                  <h2 class="text-2xl font-semibold text-gray-900 dark:text-gray-100 transition-colors duration-300">已完成任务</h2>
                  <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">共 {{ completedTasks.length }} 个已完成任务</p>
                </div>
              </div>
            </div>
            
            <!-- 搜索过滤组件 -->
            <SearchFilter 
              :tasks="completedTasks"
              @filter="handleCompletedFilter"
            />
            
            <div v-if="loading" class="text-center py-16">
              <div class="inline-flex items-center space-x-3">
                <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-primary-500"></div>
                <span class="text-gray-500 dark:text-gray-400 transition-colors duration-300">加载中...</span>
              </div>
            </div>
            
            <div v-else-if="filteredCompletedTasks.length === 0 && completedTasks.length === 0" class="text-center py-16">
              <div class="w-20 h-20 bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-800 dark:to-gray-700 rounded-2xl flex items-center justify-center mx-auto mb-6 transition-colors duration-300">
                <span class="text-3xl">🎯</span>
              </div>
              <div class="text-gray-600 dark:text-gray-300 transition-colors duration-300">
                <h3 class="text-lg font-medium mb-2">还没有完成的任务</h3>
                <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">完成一些任务后，它们会显示在这里</p>
              </div>
            </div>

            <div v-else-if="filteredCompletedTasks.length === 0" class="text-center py-16">
              <div class="w-20 h-20 bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-800 dark:to-gray-700 rounded-2xl flex items-center justify-center mx-auto mb-6 transition-colors duration-300">
                <span class="text-3xl">🔍</span>
              </div>
              <div class="text-gray-600 dark:text-gray-300 transition-colors duration-300">
                <h3 class="text-lg font-medium mb-2">没有找到匹配的任务</h3>
                <p class="text-sm text-gray-500 dark:text-gray-400 transition-colors duration-300">尝试调整搜索条件或筛选器</p>
              </div>
            </div>
            
            <div v-else class="space-y-4">
              <TaskCard 
                v-for="(task, index) in filteredCompletedTasks"
                :key="task.id"
                :task="task"
                :index="index"
                @toggle="toggleTaskCompletion"
                @edit="showTaskForm"
                @delete="deleteTask"
              />
            </div>
          </div>
          
          <!-- 设置页面 -->
          <div v-else-if="activeView === 'settings'">
            <Settings @dataChanged="refreshAllData" />
          </div>
        </div>
      </div>
    </main>
    
    <!-- 任务表单模态框 -->
    <TaskForm 
      :show="showForm"
      :task="editingTask"
      @close="hideTaskForm"
      @submit="handleTaskSubmit"
    />

    <!-- 关闭行为询问对话框 -->
    <div v-if="showCloseDialog" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 max-w-md w-mx-4 transition-colors duration-300">
        <div class="flex items-center mb-4">
          <div class="w-10 h-10 bg-yellow-100 dark:bg-yellow-900/30 rounded-full flex items-center justify-center mr-3">
            <svg class="w-6 h-6 text-yellow-600 dark:text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">关闭程序</h3>
        </div>
        
        <p class="text-gray-600 dark:text-gray-300 mb-6">您想要如何关闭程序？</p>
        
        <div class="space-y-3 mb-6">
          <label class="flex items-center">
            <input type="radio" v-model="closeDialogChoice" value="exit" class="mr-3">
            <span class="text-gray-900 dark:text-gray-100">完全退出程序</span>
          </label>
          <label class="flex items-center">
            <input type="radio" v-model="closeDialogChoice" value="minimize" class="mr-3">
            <span class="text-gray-900 dark:text-gray-100">最小化到系统托盘</span>
          </label>
        </div>
        
        <div class="flex items-center mb-6">
          <input type="checkbox" v-model="rememberChoice" class="mr-2">
          <span class="text-sm text-gray-600 dark:text-gray-400">记住我的选择</span>
        </div>
        
        <div class="flex space-x-3">
          <button 
            @click="handleCloseDialogAction"
            :disabled="!closeDialogChoice"
            class="flex-1 px-4 py-2 bg-primary-500 text-white rounded-md hover:bg-primary-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-300"
          >
            确定
          </button>
          <button 
            @click="showCloseDialog = false"
            class="flex-1 px-4 py-2 bg-gray-300 dark:bg-gray-600 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-400 dark:hover:bg-gray-500 transition-colors duration-300"
          >
            取消
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import Navigation from './components/Navigation.vue';
import TaskCard from './components/TaskCard.vue';
import TaskForm from './components/TaskForm.vue';
import Settings from './components/Settings.vue';
import SearchFilter from './components/SearchFilter.vue';
import ThemeToggle from './components/ThemeToggle.vue';
import { TaskAPI } from './utils/taskAPI';
import type { Task, CreateTaskRequest, UpdateTaskRequest } from './types/task';
import { useTheme } from './composables/useTheme';
import { useNotifications } from './composables/useNotifications';

// 响应式数据
const activeView = ref('today');
const loading = ref(false);
const showForm = ref(false);
const editingTask = ref<Task | null>(null);

// 关闭对话框相关状态
const showCloseDialog = ref(false);
const closeDialogChoice = ref('');
const rememberChoice = ref(false);

const allTasks = ref<Task[]>([]);
const todayTasks = ref<Task[]>([]);
const completedTasks = ref<Task[]>([]);

// 筛选后的任务数据
const filteredTodayTasks = ref<Task[]>([]);
const filteredAllTasks = ref<Task[]>([]);
const filteredCompletedTasks = ref<Task[]>([]);

// 计算属性
const activeTasks = computed(() => 
  allTasks.value.filter(task => !task.is_completed)
);

// 筛选事件处理
const handleTodayFilter = (filtered: Task[]) => {
  filteredTodayTasks.value = filtered;
};

const handleAllFilter = (filtered: Task[]) => {
  filteredAllTasks.value = filtered;
};

const handleCompletedFilter = (filtered: Task[]) => {
  filteredCompletedTasks.value = filtered;
};

// 刷新所有数据
const refreshAllData = async () => {
  loading.value = true;
  try {
    await Promise.all([
      refreshAllTasks(),
      refreshTodayTasks(),
      refreshCompletedTasks()
    ]);
  } catch (error) {
    console.error('获取数据失败:', error);
    alert('获取数据失败: ' + (error as Error).message);
  } finally {
    loading.value = false;
  }
};

// 刷新所有任务
const refreshAllTasks = async () => {
  try {
    allTasks.value = await TaskAPI.getAllTasks();
  } catch (error) {
    console.error('获取所有任务失败:', error);
    throw error;
  }
};

// 刷新今日任务
const refreshTodayTasks = async () => {
  try {
    todayTasks.value = await TaskAPI.getTodayTasks();
  } catch (error) {
    console.error('获取今日任务失败:', error);
    throw error;
  }
};

// 刷新已完成任务
const refreshCompletedTasks = async () => {
  try {
    completedTasks.value = await TaskAPI.getCompletedTasks();
  } catch (error) {
    console.error('获取已完成任务失败:', error);
    throw error;
  }
};

// 初始化主题
const { initTheme } = useTheme();
const { notifyTaskCompleted, clearTaskNotification, resetNotifications } = useNotifications();

let cleanupTheme: (() => void) | undefined;
let unlisten: (() => void) | undefined;

// 页面加载时获取数据
onMounted(() => {
  // 初始化主题系统
  cleanupTheme = initTheme();
  
  // 监听关闭事件
  setupCloseEventListener();
  
  // 始终尝试加载数据，不检查环境
  refreshAllData();
});

onUnmounted(() => {
  // 清理主题监听器
  if (cleanupTheme) {
    cleanupTheme();
  }
  
  // 清理关闭事件监听器
  if (unlisten) {
    unlisten();
  }
});

// 设置关闭事件监听器
const setupCloseEventListener = async () => {
  try {
    unlisten = await listen('close-requested', handleCloseRequested);
    
    // 监听托盘菜单事件
    await listen('show-add-task', () => {
      showTaskForm();
    });
    
    await listen('show-settings', () => {
      activeView.value = 'settings';
    });
  } catch (error) {
    console.error('设置事件监听器失败:', error);
  }
};

// 处理关闭请求
const handleCloseRequested = async () => {
  try {
    const behavior = await TaskAPI.handleCloseRequest();
    
    if (behavior === 'ask') {
      // 显示询问对话框
      showCloseDialog.value = true;
      closeDialogChoice.value = '';
      rememberChoice.value = false;
    }
    // 如果是 'exit' 或 'minimize'，后端已经处理了
  } catch (error) {
    console.error('处理关闭请求失败:', error);
  }
};

// 处理关闭对话框的操作
const handleCloseDialogAction = async () => {
  try {
    if (rememberChoice.value) {
      // 更新用户设置
      await TaskAPI.updateAppSettings({
        close_behavior: closeDialogChoice.value
      });
    }
    
    // 根据选择执行操作
    if (closeDialogChoice.value === 'exit') {
      await TaskAPI.forceExitApp();
    } else if (closeDialogChoice.value === 'minimize') {
      await TaskAPI.minimizeToTray();
    }
    
    showCloseDialog.value = false;
  } catch (error) {
    console.error('执行关闭操作失败:', error);
  }
};

// 显示任务表单
const showTaskForm = (task?: Task) => {
  editingTask.value = task || null;
  showForm.value = true;
};

// 隐藏任务表单
const hideTaskForm = () => {
  showForm.value = false;
  editingTask.value = null;
};

// 处理任务表单提交
const handleTaskSubmit = async (data: CreateTaskRequest | UpdateTaskRequest) => {
  try {
    console.log('提交任务数据:', data);
    
    if ('id' in data) {
      // 更新任务
      await TaskAPI.updateTask(data);
    } else {
      // 创建任务
      await TaskAPI.createTask(data);
    }
    
    hideTaskForm();
    await refreshAllData();
  } catch (error) {
    console.error('保存任务失败:', error);
    console.error('错误详情:', JSON.stringify(error));
    alert('保存任务失败: ' + (error as Error).message);
  }
};

// 切换任务完成状态
const toggleTaskCompletion = async (task: Task) => {
  try {
    const updatedTask = await TaskAPI.toggleTaskCompletion(task.id!);
    
    // 如果任务被标记为完成，发送完成通知
    if (updatedTask.is_completed && !task.is_completed) {
      await notifyTaskCompleted(updatedTask);
    }
    
    await refreshAllData();
  } catch (error) {
    console.error('更新任务状态失败:', error);
    alert('更新任务状态失败: ' + (error as Error).message);
  }
};

// 删除任务
const deleteTask = async (task: Task) => {
  try {
    await TaskAPI.deleteTask(task.id!);
    
    // 清除该任务的通知状态
    if (task.id) {
      clearTaskNotification(task.id);
    }
    
    await refreshAllData();
  } catch (error) {
    console.error('删除任务失败:', error);
    alert('删除任务失败: ' + (error as Error).message);
  }
};
</script>