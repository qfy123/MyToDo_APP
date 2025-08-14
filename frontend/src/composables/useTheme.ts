import { ref, watch } from 'vue';

export type Theme = 'light' | 'dark' | 'auto';

const THEME_STORAGE_KEY = 'mytodo-theme';

// 主题状态
const theme = ref<Theme>('auto');
const isDark = ref(false);

// 从本地存储加载主题设置
const loadTheme = () => {
  const saved = localStorage.getItem(THEME_STORAGE_KEY);
  if (saved && ['light', 'dark', 'auto'].includes(saved)) {
    theme.value = saved as Theme;
  }
};

// 保存主题设置到本地存储
const saveTheme = (newTheme: Theme) => {
  localStorage.setItem(THEME_STORAGE_KEY, newTheme);
};

// 检测系统主题
const getSystemTheme = (): 'light' | 'dark' => {
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
};

// 应用主题到 DOM
const applyTheme = (targetTheme: 'light' | 'dark') => {
  const html = document.documentElement;
  
  if (targetTheme === 'dark') {
    html.classList.add('dark');
    isDark.value = true;
  } else {
    html.classList.remove('dark');
    isDark.value = false;
  }
};

// 更新主题
const updateTheme = () => {
  let targetTheme: 'light' | 'dark';
  
  if (theme.value === 'auto') {
    targetTheme = getSystemTheme();
  } else {
    targetTheme = theme.value;
  }
  
  applyTheme(targetTheme);
};

// 设置主题
const setTheme = (newTheme: Theme) => {
  theme.value = newTheme;
  saveTheme(newTheme);
  updateTheme();
};

// 切换主题（在 light/dark 之间切换）
const toggleTheme = () => {
  if (theme.value === 'auto') {
    // 如果当前是自动模式，切换到与系统相反的模式
    const systemTheme = getSystemTheme();
    setTheme(systemTheme === 'dark' ? 'light' : 'dark');
  } else {
    // 在 light 和 dark 之间切换
    setTheme(theme.value === 'dark' ? 'light' : 'dark');
  }
};

// 监听系统主题变化
const setupSystemThemeListener = () => {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  
  const handleSystemThemeChange = () => {
    if (theme.value === 'auto') {
      updateTheme();
    }
  };
  
  mediaQuery.addEventListener('change', handleSystemThemeChange);
  
  // 返回清理函数
  return () => {
    mediaQuery.removeEventListener('change', handleSystemThemeChange);
  };
};

// 监听主题变化
watch(theme, updateTheme);

// 初始化主题
const initTheme = () => {
  loadTheme();
  updateTheme();
  return setupSystemThemeListener();
};

// Composable hook
export const useTheme = () => {
  return {
    theme: theme,
    isDark: isDark,
    setTheme,
    toggleTheme,
    initTheme,
    getSystemTheme
  };
};