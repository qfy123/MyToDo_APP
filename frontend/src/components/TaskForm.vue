<template>
  <!-- 模态框遮罩 -->
  <div 
    v-if="show" 
    class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center z-50 transition-colors duration-300"
    @click="closeModal"
  >
    <!-- 表单容器 -->
    <div 
      class="bg-white dark:bg-gray-800 rounded-lg shadow-lg dark:shadow-lg w-full max-w-md mx-4 max-h-[90vh] overflow-y-auto transition-colors duration-300"
      @click.stop
    >
      <!-- 表单头部 -->
      <div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700 transition-colors duration-300">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 transition-colors duration-300">
          {{ isEdit ? '编辑任务' : '创建新任务' }}
        </h3>
        <button 
          @click="closeModal"
          class="text-gray-400 dark:text-gray-500 hover:text-gray-600 dark:hover:text-gray-300 transition-colors duration-300"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      
      <!-- 表单内容 -->
      <form @submit.prevent="handleSubmit" class="p-6 space-y-4">
        <!-- 任务标题 -->
        <div>
          <label for="title" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 transition-colors duration-300">
            任务标题 <span class="text-red-500">*</span>
          </label>
          <input 
            id="title"
            v-model="form.title"
            type="text" 
            placeholder="输入任务标题..."
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400 transition-colors duration-300"
            required
            maxlength="100"
          />
        </div>
        
        <!-- 任务描述 -->
        <div>
          <label for="description" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 transition-colors duration-300">
            任务描述
          </label>
          <textarea 
            id="description"
            v-model="form.description"
            placeholder="输入任务详细描述..."
            rows="3"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 resize-none bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400 transition-colors duration-300"
            maxlength="500"
          ></textarea>
          <div class="text-xs text-gray-500 dark:text-gray-400 mt-1 transition-colors duration-300">
            {{ form.description?.length || 0 }}/500
          </div>
        </div>
        
        <!-- 优先级选择 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 transition-colors duration-300">
            优先级
          </label>
          <div class="grid grid-cols-3 gap-2">
            <button 
              type="button"
              v-for="priority in priorityOptions"
              :key="priority.value"
              @click="form.priority = priority.value"
              :class="[
                'px-3 py-2 rounded-md text-sm font-medium transition-colors duration-300',
                form.priority === priority.value 
                  ? priority.selectedClass 
                  : priority.defaultClass
              ]"
            >
              {{ priority.label }}
            </button>
          </div>
        </div>
        
        <!-- 截止日期 -->
        <div>
          <label for="dueDate" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 transition-colors duration-300">
            截止日期
          </label>
          <input 
            id="dueDate"
            v-model="form.due_date"
            type="datetime-local" 
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 transition-colors duration-300"
          />
        </div>
        
        <!-- 标签 -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 transition-colors duration-300">
            标签
          </label>
          <div class="space-y-2">
            <!-- 已选标签显示 -->
            <div v-if="form.tags.length > 0" class="flex flex-wrap gap-2">
              <span 
                v-for="(tag, index) in form.tags" 
                :key="index"
                class="inline-flex items-center px-2 py-1 rounded-md text-xs font-medium bg-primary-100 dark:bg-primary-900/50 text-primary-700 dark:text-primary-300 border border-primary-200 dark:border-primary-700/50 transition-colors duration-300"
              >
                <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
                </svg>
                {{ tag }}
                <button
                  type="button"
                  @click="removeTag(index)"
                  class="ml-1 text-primary-600 dark:text-primary-400 hover:text-primary-800 dark:hover:text-primary-200 transition-colors duration-300"
                >
                  <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </span>
            </div>
            
            <!-- 标签输入 -->
            <div class="flex">
              <input 
                v-model="newTag"
                type="text" 
                placeholder="输入标签名称并按回车添加..."
                @keydown.enter.prevent="addTag"
                @keydown.comma.prevent="addTag"
                class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-l-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400 transition-colors duration-300"
                maxlength="20"
              />
              <button
                type="button"
                @click="addTag"
                class="px-3 py-2 bg-primary-500 text-white rounded-r-md hover:bg-primary-600 transition-colors duration-300"
              >
                添加
              </button>
            </div>
            <p class="text-xs text-gray-500 dark:text-gray-400 transition-colors duration-300">按回车键或逗号快速添加标签</p>
          </div>
        </div>
        
        <!-- 完成状态（仅编辑时显示） -->
        <div v-if="isEdit" class="flex items-center">
          <input 
            id="completed"
            v-model="form.is_completed"
            type="checkbox" 
            class="rounded border-gray-300 dark:border-gray-600 text-blue-600 focus:ring-blue-500 focus:ring-2 dark:focus:ring-offset-gray-800 bg-white dark:bg-gray-700 transition-colors duration-300"
          />
          <label for="completed" class="ml-2 text-sm text-gray-700 dark:text-gray-300 transition-colors duration-300">
            标记为已完成
          </label>
        </div>
        
        <!-- 表单按钮 -->
        <div class="flex justify-end space-x-3 pt-4">
          <button 
            type="button"
            @click="closeModal"
            class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-600 hover:bg-gray-200 dark:hover:bg-gray-500 rounded-md transition-colors duration-300"
          >
            取消
          </button>
          <button 
            type="submit"
            :disabled="!form.title.trim() || isSubmitting"
            class="px-4 py-2 text-sm font-medium text-white bg-blue-500 hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed rounded-md transition-colors duration-300"
          >
            {{ isSubmitting ? '提交中...' : (isEdit ? '更新任务' : '创建任务') }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch, computed } from 'vue';
import type { Task, CreateTaskRequest, UpdateTaskRequest } from '../types/task';
import { TaskPriority } from '../types/task';

interface Props {
  show: boolean;
  task?: Task | null;
}

interface Emits {
  (e: 'close'): void;
  (e: 'submit', data: CreateTaskRequest | UpdateTaskRequest): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const isSubmitting = ref(false);

// 判断是否为编辑模式
const isEdit = computed(() => props.task !== null);

// 表单数据
const form = reactive({
  title: '',
  description: '',
  priority: TaskPriority.Medium,
  due_date: '',
  is_completed: false,
  tags: [] as string[]
});

const newTag = ref('');

// 优先级选项
const priorityOptions = [
  {
    value: TaskPriority.Low,
    label: '★☆☆ 低',
    defaultClass: 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600',
    selectedClass: 'bg-green-100 dark:bg-green-900/50 text-green-800 dark:text-green-200 border-green-300 dark:border-green-700/50'
  },
  {
    value: TaskPriority.Medium,
    label: '★★☆ 中',
    defaultClass: 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600',
    selectedClass: 'bg-yellow-100 dark:bg-yellow-900/50 text-yellow-800 dark:text-yellow-200 border-yellow-300 dark:border-yellow-700/50'
  },
  {
    value: TaskPriority.High,
    label: '★★★ 高',
    defaultClass: 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600',
    selectedClass: 'bg-red-100 dark:bg-red-900/50 text-red-800 dark:text-red-200 border-red-300 dark:border-red-700/50'
  }
];

// 重置表单
const resetForm = () => {
  form.title = '';
  form.description = '';
  form.priority = TaskPriority.Medium;
  form.due_date = '';
  form.is_completed = false;
  form.tags = [];
  newTag.value = '';
};

// 添加标签
const addTag = () => {
  const tag = newTag.value.trim();
  if (tag && !form.tags.includes(tag)) {
    form.tags.push(tag);
    newTag.value = '';
  }
};

// 移除标签
const removeTag = (index: number) => {
  form.tags.splice(index, 1);
};

// 监听任务变化，更新表单
watch(() => props.task, (newTask) => {
  if (newTask) {
    // 编辑模式：填充现有任务数据
    form.title = newTask.title;
    form.description = newTask.description || '';
    
    // 确保 priority 始终是数字枚举值
    if (typeof newTask.priority === 'string') {
      // 如果是字符串，转换为对应的枚举值
      switch (newTask.priority) {
        case 'Low':
          form.priority = TaskPriority.Low;
          break;
        case 'Medium':
          form.priority = TaskPriority.Medium;
          break;
        case 'High':
          form.priority = TaskPriority.High;
          break;
        default:
          form.priority = TaskPriority.Medium;
      }
    } else {
      // 如果已经是数字，直接使用
      form.priority = newTask.priority;
    }
    
    form.due_date = newTask.due_date ? formatDateTimeLocal(newTask.due_date) : '';
    form.is_completed = newTask.is_completed;
    form.tags = [...(newTask.tags || [])]; // 复制标签数组
  } else {
    // 创建模式：重置表单
    resetForm();
  }
}, { immediate: true });

// 监听显示状态，重置表单
watch(() => props.show, (show) => {
  if (show && !props.task) {
    resetForm();
  }
});

// 格式化日期为本地输入格式
const formatDateTimeLocal = (dateString: string) => {
  const date = new Date(dateString);
  // 格式：YYYY-MM-DDTHH:mm
  return date.toISOString().slice(0, 16);
};

// 处理表单提交
const handleSubmit = async () => {
  if (!form.title.trim()) return;
  
  isSubmitting.value = true;
  
  try {
    if (isEdit.value && props.task) {
      // 编辑模式
      const updateData: UpdateTaskRequest = {
        id: props.task.id!,
        title: form.title.trim(),
        description: form.description.trim() || undefined,
        priority: form.priority,
        due_date: form.due_date ? new Date(form.due_date).toISOString() : undefined,
        is_completed: form.is_completed,
        tags: form.tags.length > 0 ? form.tags : undefined
      };
      emit('submit', updateData);
    } else {
      // 创建模式
      const createData: CreateTaskRequest = {
        title: form.title.trim(),
        description: form.description.trim() || undefined,
        priority: form.priority,
        due_date: form.due_date ? new Date(form.due_date).toISOString() : undefined,
        tags: form.tags.length > 0 ? form.tags : undefined
      };
      emit('submit', createData);
    }
  } catch (error) {
    console.error('提交表单时出错:', error);
  } finally {
    isSubmitting.value = false;
  }
};

// 关闭模态框
const closeModal = () => {
  if (!isSubmitting.value) {
    emit('close');
  }
};
</script>