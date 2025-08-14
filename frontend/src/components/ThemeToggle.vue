<template>
  <div class="relative">
    <!-- 主题切换按钮 -->
    <button
      @click="toggleDropdown"
      ref="buttonRef"
      class="
        p-2 rounded-lg transition-all duration-200
        bg-white/80 hover:bg-white dark:bg-gray-800/80 dark:hover:bg-gray-800
        border border-gray-200 dark:border-gray-700
        text-gray-700 dark:text-gray-300
        hover:text-gray-900 dark:hover:text-gray-100
        shadow-sm hover:shadow-md
        focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2
        dark:focus:ring-offset-gray-900
      "
      :title="getThemeTitle()"
    >
      <!-- 主题图标 -->
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <!-- 太阳图标 (浅色模式) -->
        <path
          v-if="!isDark"
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
        />
        <!-- 月亮图标 (深色模式) -->
        <path
          v-else
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
        />
      </svg>
    </button>

    <!-- 下拉菜单 -->
    <Transition
      enter-active-class="transition ease-out duration-100"
      enter-from-class="transform opacity-0 scale-95"
      enter-to-class="transform opacity-100 scale-100"
      leave-active-class="transition ease-in duration-75"
      leave-from-class="transform opacity-100 scale-100"
      leave-to-class="transform opacity-0 scale-95"
    >
      <div
        v-if="showDropdown"
        ref="dropdownRef"
        class="
          absolute right-0 top-full mt-2 w-48 z-50
          bg-white dark:bg-gray-800
          rounded-lg shadow-lg border border-gray-200 dark:border-gray-700
          py-2
        "
      >
        <!-- 主题选项 -->
        <button
          v-for="option in themeOptions"
          :key="option.value"
          @click="selectTheme(option.value)"
          class="
            w-full px-4 py-2 text-left text-sm transition-colors
            hover:bg-gray-100 dark:hover:bg-gray-700
            flex items-center space-x-3
            text-gray-700 dark:text-gray-300
          "
          :class="{
            'bg-primary-50 dark:bg-primary-900/30 text-primary-600 dark:text-primary-400': theme === option.value
          }"
        >
          <component :is="option.icon" class="w-4 h-4 flex-shrink-0" />
          <span class="flex-1">{{ option.label }}</span>
          <svg
            v-if="theme === option.value"
            class="w-4 h-4 text-primary-600 dark:text-primary-400"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fill-rule="evenodd"
              d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
              clip-rule="evenodd"
            />
          </svg>
        </button>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, h } from 'vue';
import { useTheme, type Theme } from '../composables/useTheme';

const { theme, isDark, setTheme, getSystemTheme } = useTheme();

const showDropdown = ref(false);
const buttonRef = ref<HTMLButtonElement>();
const dropdownRef = ref<HTMLDivElement>();

// 主题选项
const themeOptions = [
  {
    value: 'light' as Theme,
    label: '浅色模式',
    icon: () => h('svg', {
      class: 'w-4 h-4',
      fill: 'none',
      stroke: 'currentColor',
      viewBox: '0 0 24 24'
    }, [
      h('path', {
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'stroke-width': '2',
        d: 'M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z'
      })
    ])
  },
  {
    value: 'dark' as Theme,
    label: '深色模式',
    icon: () => h('svg', {
      class: 'w-4 h-4',
      fill: 'none',
      stroke: 'currentColor',
      viewBox: '0 0 24 24'
    }, [
      h('path', {
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'stroke-width': '2',
        d: 'M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z'
      })
    ])
  },
  {
    value: 'auto' as Theme,
    label: '跟随系统',
    icon: () => h('svg', {
      class: 'w-4 h-4',
      fill: 'none',
      stroke: 'currentColor',
      viewBox: '0 0 24 24'
    }, [
      h('path', {
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round',
        'stroke-width': '2',
        d: 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z'
      })
    ])
  }
];

// 切换下拉菜单
const toggleDropdown = () => {
  showDropdown.value = !showDropdown.value;
};

// 选择主题
const selectTheme = (selectedTheme: Theme) => {
  setTheme(selectedTheme);
  showDropdown.value = false;
};

// 获取主题标题
const getThemeTitle = () => {
  const option = themeOptions.find(opt => opt.value === theme.value);
  return option ? `当前主题: ${option.label}` : '切换主题';
};

// 点击外部关闭下拉菜单
const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as Node;
  
  if (
    showDropdown.value &&
    buttonRef.value &&
    dropdownRef.value &&
    !buttonRef.value.contains(target) &&
    !dropdownRef.value.contains(target)
  ) {
    showDropdown.value = false;
  }
};

// 键盘事件处理
const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && showDropdown.value) {
    showDropdown.value = false;
    buttonRef.value?.focus();
  }
};

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
  document.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside);
  document.removeEventListener('keydown', handleKeydown);
});
</script>