<template>
  <div 
    :class="[
      'group bg-white/80 dark:bg-gray-800/80 backdrop-blur-sm border border-white/50 dark:border-gray-700/50 rounded-xl p-5 transition-all duration-300 hover:shadow-medium dark:hover:shadow-medium-dark hover:bg-white/90 dark:hover:bg-gray-800/90',
      task.is_completed 
        ? 'opacity-75 hover:opacity-85' 
        : 'hover:-translate-y-0.5'
    ]"
  >
    <div class="flex items-start justify-between">
      <!-- 左侧：复选框和任务内容 -->
      <div class="flex items-start space-x-4 flex-1">
        <div class="mt-1">
          <input 
            type="checkbox" 
            :checked="task.is_completed"
            @change="toggleCompletion"
            class="w-5 h-5 rounded-lg border-2 border-gray-300 dark:border-gray-600 text-primary-500 focus:ring-primary-500 focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-gray-800 transition-colors duration-300 bg-white dark:bg-gray-700"
          />
        </div>
        
        <div class="flex-1 min-w-0">
          <!-- 任务标题 -->
          <h3 :class="[
            'font-semibold text-gray-900 dark:text-gray-100 break-words text-lg mb-2 transition-colors duration-300',
            task.is_completed ? 'line-through text-gray-500 dark:text-gray-500' : 'group-hover:text-gray-800 dark:group-hover:text-gray-200'
          ]">
            {{ task.title }}
          </h3>
          
          <!-- 任务描述 -->
          <p 
            v-if="task.description" 
            :class="[
              'text-gray-600 dark:text-gray-400 break-words transition-colors duration-300',
              task.is_completed ? 'line-through text-gray-400 dark:text-gray-500' : 'group-hover:text-gray-700 dark:group-hover:text-gray-300',
              showFullDescription ? '' : 'line-clamp-2'
            ]"
            @click="toggleDescription"
            :title="task.description.length > 100 ? '点击展开/收起' : ''"
          >
            {{ task.description }}
          </p>
          
          <!-- 截止日期和创建时间 -->
          <div class="flex items-center space-x-4 mt-3">
            <span v-if="task.due_date" :class="[
              'inline-flex items-center px-3 py-1 rounded-lg text-sm font-medium transition-colors duration-300',
              isOverdue && !task.is_completed 
                ? 'bg-red-100 dark:bg-red-900/30 text-red-800 dark:text-red-200 border border-red-200 dark:border-red-700/50' : 
              isDueToday && !task.is_completed 
                ? 'bg-yellow-100 dark:bg-yellow-900/30 text-yellow-800 dark:text-yellow-200 border border-yellow-200 dark:border-yellow-700/50' 
                : 'bg-gray-100 dark:bg-gray-700/50 text-gray-700 dark:text-gray-300 border border-gray-200 dark:border-gray-600/50'
            ]">
              <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              {{ formatDate(task.due_date) }}
            </span>
            <span class="text-sm text-gray-500 dark:text-gray-400 flex items-center transition-colors duration-300">
              <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              创建于 {{ formatDate(task.created_at) }}
            </span>
          </div>
          
          <!-- 标签显示 -->
          <div v-if="task.tags && task.tags.length > 0" class="flex flex-wrap gap-2 mt-3">
            <span 
              v-for="tag in task.tags" 
              :key="tag"
              class="inline-flex items-center px-2 py-1 rounded-md text-xs font-medium bg-primary-100 dark:bg-primary-900/50 text-primary-700 dark:text-primary-300 border border-primary-200 dark:border-primary-700/50 transition-colors duration-300"
            >
              <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
              </svg>
              {{ tag }}
            </span>
          </div>
        </div>
      </div>
      
      <!-- 右侧：优先级和操作按钮 -->
      <div class="flex items-start space-x-3 ml-4">
        <!-- 优先级显示 -->
        <span :class="[
          'inline-flex items-center px-3 py-1 rounded-lg text-sm font-medium border transition-colors duration-300',
          getPriorityStyle(task.priority)
        ]">
          {{ getPriorityText(task.priority) }}
        </span>
        
        <!-- 操作按钮 -->
        <div class="flex items-center space-x-1 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
          <button 
            @click="$emit('edit', task)"
            class="p-2 text-gray-400 dark:text-gray-500 hover:text-primary-600 dark:hover:text-primary-400 hover:bg-primary-50 dark:hover:bg-primary-900/30 rounded-lg transition-all duration-200"
            title="编辑任务"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
          </button>
          
          <button 
            @click="confirmDelete"
            class="p-2 text-gray-400 dark:text-gray-500 hover:text-red-600 dark:hover:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/30 rounded-lg transition-all duration-200"
            title="删除任务"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import type { Task } from '../types/task';
import { TaskPriority } from '../types/task';

interface Props {
  task: Task;
  index: number;
}

interface Emits {
  (e: 'toggle', task: Task): void;
  (e: 'edit', task: Task): void;
  (e: 'delete', task: Task): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const showFullDescription = ref(false);

// 计算是否过期和是否今日到期
const isOverdue = computed(() => {
  if (!props.task.due_date) return false;
  const dueDate = new Date(props.task.due_date);
  const today = new Date();
  today.setHours(0, 0, 0, 0);
  return dueDate < today && !props.task.is_completed;
});

const isDueToday = computed(() => {
  if (!props.task.due_date) return false;
  const dueDate = new Date(props.task.due_date);
  const today = new Date();
  return dueDate.toDateString() === today.toDateString();
});

// 格式化日期
const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  const now = new Date();
  const diffTime = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));
  
  if (diffDays === 0) {
    return '今天 ' + date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
  } else if (diffDays === 1) {
    return '昨天';
  } else if (diffDays === -1) {
    return '明天';
  } else if (diffDays > 1 && diffDays < 7) {
    return `${diffDays}天前`;
  } else if (diffDays < -1 && diffDays > -7) {
    return `${Math.abs(diffDays)}天后`;
  } else {
    return date.toLocaleDateString('zh-CN', { 
      month: 'short', 
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }
};

// 获取优先级样式
const getPriorityStyle = (priority: TaskPriority) => {
  switch (priority) {
    case TaskPriority.High:
      return 'bg-red-50 dark:bg-red-900/30 text-red-700 dark:text-red-200 border-red-200 dark:border-red-700/50';
    case TaskPriority.Medium:
      return 'bg-yellow-50 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-200 border-yellow-200 dark:border-yellow-700/50';
    case TaskPriority.Low:
      return 'bg-green-50 dark:bg-green-900/30 text-green-700 dark:text-green-200 border-green-200 dark:border-green-700/50';
    default:
      return 'bg-gray-50 dark:bg-gray-700/50 text-gray-700 dark:text-gray-300 border-gray-200 dark:border-gray-600/50';
  }
};

// 获取优先级文本
const getPriorityText = (priority: TaskPriority) => {
  switch (priority) {
    case TaskPriority.High:
      return '🔴 高优先级';
    case TaskPriority.Medium:
      return '🟡 中优先级';
    case TaskPriority.Low:
      return '🟢 低优先级';
    default:
      return '🟡 中优先级';
  }
};

// 切换完成状态
const toggleCompletion = () => {
  emit('toggle', props.task);
};

// 切换描述显示
const toggleDescription = () => {
  if (props.task.description && props.task.description.length > 100) {
    showFullDescription.value = !showFullDescription.value;
  }
};

// 确认删除
const confirmDelete = () => {
  if (confirm(`确定要删除任务"${props.task.title}"吗？此操作无法撤销。`)) {
    emit('delete', props.task);
  }
};
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>