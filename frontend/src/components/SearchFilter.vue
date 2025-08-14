<template>
  <div class="bg-white/80 dark:bg-gray-800/80 backdrop-blur-sm rounded-xl shadow-soft dark:shadow-soft-dark border border-white/50 dark:border-gray-700/50 p-6 mb-6 transition-colors duration-300">
    <div class="flex flex-col space-y-4 lg:flex-row lg:space-y-0 lg:space-x-4">
      <!-- 搜索框 -->
      <div class="flex-1">
        <div class="relative">
          <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <svg class="h-5 w-5 text-gray-400 dark:text-gray-500 transition-colors duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
          </div>
          <input
            v-model="searchText"
            type="text"
            placeholder="搜索任务标题、描述或标签..."
            class="block w-full pl-10 pr-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 bg-white/90 dark:bg-gray-700/90 text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400 transition-colors duration-300"
            @input="onSearchChange"
          />
          <button
            v-if="searchText"
            @click="clearSearch"
            class="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 dark:text-gray-500 hover:text-gray-600 dark:hover:text-gray-300 transition-colors duration-300"
          >
            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>

      <!-- 优先级过滤 -->
      <div class="flex-shrink-0">
        <select
          v-model="selectedPriority"
          @change="onFilterChange"
          class="block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 bg-white/90 dark:bg-gray-700/90 text-gray-900 dark:text-gray-100 transition-colors duration-300"
        >
          <option value="">所有优先级</option>
          <option value="2">★★★ 高优先级</option>
          <option value="1">★★☆ 中优先级</option>
          <option value="0">★☆☆ 低优先级</option>
        </select>
      </div>

      <!-- 标签过滤 -->
      <div class="flex-shrink-0">
        <select
          v-model="selectedTag"
          @change="onFilterChange"
          class="block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 bg-white/90 dark:bg-gray-700/90 text-gray-900 dark:text-gray-100 transition-colors duration-300"
        >
          <option value="">所有标签</option>
          <option
            v-for="tag in availableTags"
            :key="tag"
            :value="tag"
          >
            🏷️ {{ tag }}
          </option>
        </select>
      </div>

      <!-- 排序选择 -->
      <div class="flex-shrink-0">
        <select
          v-model="sortBy"
          @change="onFilterChange"
          class="block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 bg-white/90 dark:bg-gray-700/90 text-gray-900 dark:text-gray-100 transition-colors duration-300"
        >
          <option value="created_at">按创建时间</option>
          <option value="due_date">按截止时间</option>
          <option value="priority">按优先级</option>
          <option value="title">按标题</option>
        </select>
      </div>

      <!-- 排序方向 -->
      <div class="flex-shrink-0">
        <button
          @click="toggleSortOrder"
          class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 bg-white/90 dark:bg-gray-700/90 transition-colors duration-300"
          :title="sortOrder === 'asc' ? '升序' : '降序'"
        >
          <svg
            class="h-5 w-5 text-gray-600 dark:text-gray-300 transition-colors duration-300"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            :class="{ 'transform rotate-180': sortOrder === 'desc' }"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4h13M3 8h9m-9 4h6m4 0l4-4m0 0l4 4m-4-4v12" />
          </svg>
        </button>
      </div>

      <!-- 清除按钮 -->
      <div class="flex-shrink-0">
        <button
          @click="clearAllFilters"
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-600 hover:bg-gray-200 dark:hover:bg-gray-500 rounded-lg transition-colors duration-300"
          :disabled="!hasActiveFilters"
        >
          清除筛选
        </button>
      </div>
    </div>

    <!-- 筛选状态显示 -->
    <div v-if="hasActiveFilters" class="mt-4 flex flex-wrap gap-2">
      <span
        v-if="searchText"
        class="inline-flex items-center px-2 py-1 rounded-md text-xs font-medium bg-blue-100 dark:bg-blue-900/50 text-blue-800 dark:text-blue-200 transition-colors duration-300"
      >
        搜索: {{ searchText }}
        <button @click="clearSearch" class="ml-1 text-blue-600 dark:text-blue-300 hover:text-blue-800 dark:hover:text-blue-100 transition-colors duration-300">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </span>
      
      <span
        v-if="selectedPriority"
        class="inline-flex items-center px-2 py-1 rounded-md text-xs font-medium bg-green-100 dark:bg-green-900/50 text-green-800 dark:text-green-200 transition-colors duration-300"
      >
        优先级: {{ getPriorityLabel(selectedPriority) }}
        <button @click="selectedPriority = ''; onFilterChange()" class="ml-1 text-green-600 dark:text-green-300 hover:text-green-800 dark:hover:text-green-100 transition-colors duration-300">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </span>

      <span
        v-if="selectedTag"
        class="inline-flex items-center px-2 py-1 rounded-md text-xs font-medium bg-purple-100 dark:bg-purple-900/50 text-purple-800 dark:text-purple-200 transition-colors duration-300"
      >
        标签: {{ selectedTag }}
        <button @click="selectedTag = ''; onFilterChange()" class="ml-1 text-purple-600 dark:text-purple-300 hover:text-purple-800 dark:hover:text-purple-100 transition-colors duration-300">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import type { Task } from '../types/task';

interface Props {
  tasks: Task[];
}

interface Emits {
  (e: 'filter', filteredTasks: Task[]): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// 搜索和过滤状态
const searchText = ref('');
const selectedPriority = ref('');
const selectedTag = ref('');
const sortBy = ref('created_at');
const sortOrder = ref<'asc' | 'desc'>('desc');

// 计算可用标签
const availableTags = computed(() => {
  const tags = new Set<string>();
  props.tasks.forEach(task => {
    if (task.tags) {
      task.tags.forEach(tag => tags.add(tag));
    }
  });
  return Array.from(tags).sort();
});

// 检查是否有活跃的筛选条件
const hasActiveFilters = computed(() => {
  return !!(searchText.value || selectedPriority.value || selectedTag.value);
});

// 获取优先级标签
const getPriorityLabel = (priority: string) => {
  switch (priority) {
    case '2': return '★★★ 高';
    case '1': return '★★☆ 中';
    case '0': return '★☆☆ 低';
    default: return priority;
  }
};

// 搜索函数
const searchTasks = (tasks: Task[], query: string): Task[] => {
  if (!query.trim()) return tasks;
  
  const lowercaseQuery = query.toLowerCase();
  return tasks.filter(task => {
    // 搜索标题
    if (task.title.toLowerCase().includes(lowercaseQuery)) return true;
    
    // 搜索描述
    if (task.description && task.description.toLowerCase().includes(lowercaseQuery)) return true;
    
    // 搜索标签
    if (task.tags && task.tags.some(tag => tag.toLowerCase().includes(lowercaseQuery))) return true;
    
    return false;
  });
};

// 过滤函数
const filterTasks = (tasks: Task[]): Task[] => {
  let filtered = [...tasks];
  
  // 按优先级过滤
  if (selectedPriority.value) {
    const priority = parseInt(selectedPriority.value);
    filtered = filtered.filter(task => task.priority === priority);
  }
  
  // 按标签过滤
  if (selectedTag.value) {
    filtered = filtered.filter(task => 
      task.tags && task.tags.includes(selectedTag.value)
    );
  }
  
  return filtered;
};

// 排序函数
const sortTasks = (tasks: Task[]): Task[] => {
  const sorted = [...tasks];
  
  sorted.sort((a, b) => {
    let aValue: any;
    let bValue: any;
    
    switch (sortBy.value) {
      case 'title':
        aValue = a.title.toLowerCase();
        bValue = b.title.toLowerCase();
        break;
      case 'priority':
        aValue = a.priority;
        bValue = b.priority;
        break;
      case 'due_date':
        aValue = a.due_date ? new Date(a.due_date).getTime() : 0;
        bValue = b.due_date ? new Date(b.due_date).getTime() : 0;
        break;
      case 'created_at':
      default:
        aValue = new Date(a.created_at).getTime();
        bValue = new Date(b.created_at).getTime();
        break;
    }
    
    if (aValue < bValue) return sortOrder.value === 'asc' ? -1 : 1;
    if (aValue > bValue) return sortOrder.value === 'asc' ? 1 : -1;
    return 0;
  });
  
  return sorted;
};

// 应用所有筛选和排序
const applyFilters = () => {
  let result = [...props.tasks];
  
  // 搜索
  result = searchTasks(result, searchText.value);
  
  // 过滤
  result = filterTasks(result);
  
  // 排序
  result = sortTasks(result);
  
  emit('filter', result);
};

// 事件处理函数
const onSearchChange = () => {
  applyFilters();
};

const onFilterChange = () => {
  applyFilters();
};

const toggleSortOrder = () => {
  sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc';
  applyFilters();
};

const clearSearch = () => {
  searchText.value = '';
  applyFilters();
};

const clearAllFilters = () => {
  searchText.value = '';
  selectedPriority.value = '';
  selectedTag.value = '';
  sortBy.value = 'created_at';
  sortOrder.value = 'desc';
  applyFilters();
};

// 监听任务变化，重新应用筛选
watch(() => props.tasks, () => {
  applyFilters();
}, { immediate: true });
</script>