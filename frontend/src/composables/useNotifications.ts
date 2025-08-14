import { ref, onMounted, onUnmounted } from 'vue';
import { TaskAPI } from '../utils/taskAPI';
import type { Task } from '../types/task';

export function useNotifications() {
  const isEnabled = ref(true);
  const checkInterval = ref<number | null>(null);
  const notifiedTasks = ref(new Set<number>());

  // 发送通知
  const sendNotification = async (title: string, body: string) => {
    try {
      if (isEnabled.value) {
        await TaskAPI.sendNotification(title, body);
      }
    } catch (error) {
      console.error('发送通知失败:', error);
    }
  };

  // 检查过期任务
  const checkOverdueTasks = async () => {
    try {
      const settings = await TaskAPI.getAppSettings();
      if (!settings.notifications_enabled) return;

      const overdueTasks = await TaskAPI.checkOverdueTasks();
      
      for (const task of overdueTasks) {
        if (task.id && !notifiedTasks.value.has(task.id)) {
          await sendNotification(
            '任务已过期！',
            `任务"${task.title}"已过期，请及时处理。`
          );
          notifiedTasks.value.add(task.id);
        }
      }
    } catch (error) {
      console.error('检查过期任务失败:', error);
    }
  };

  // 检查即将到期的任务
  const checkUpcomingTasks = async () => {
    try {
      const settings = await TaskAPI.getAppSettings();
      if (!settings.notifications_enabled) return;

      const upcomingTasks = await TaskAPI.getUpcomingTasks(settings.notification_time_before);
      
      for (const task of upcomingTasks) {
        if (task.id && !notifiedTasks.value.has(task.id)) {
          const minutesLeft = task.due_date 
            ? Math.floor((new Date(task.due_date).getTime() - new Date().getTime()) / (1000 * 60))
            : 0;
          
          await sendNotification(
            '任务即将到期！',
            `任务"${task.title}"将在 ${minutesLeft} 分钟后到期。`
          );
          notifiedTasks.value.add(task.id);
        }
      }
    } catch (error) {
      console.error('检查即将到期任务失败:', error);
    }
  };

  // 任务完成通知
  const notifyTaskCompleted = async (task: Task) => {
    try {
      const settings = await TaskAPI.getAppSettings();
      if (!settings.notifications_enabled) return;

      await sendNotification(
        '任务已完成！',
        `恭喜！您已完成任务"${task.title}"。`
      );
    } catch (error) {
      console.error('发送完成通知失败:', error);
    }
  };

  // 启动定期检查
  const startPeriodicCheck = () => {
    // 立即检查一次
    checkOverdueTasks();
    checkUpcomingTasks();

    // 每5分钟检查一次
    checkInterval.value = window.setInterval(() => {
      checkOverdueTasks();
      checkUpcomingTasks();
    }, 5 * 60 * 1000);
  };

  // 停止定期检查
  const stopPeriodicCheck = () => {
    if (checkInterval.value) {
      clearInterval(checkInterval.value);
      checkInterval.value = null;
    }
  };

  // 清除任务的通知状态（当任务被修改或删除时调用）
  const clearTaskNotification = (taskId: number) => {
    notifiedTasks.value.delete(taskId);
  };

  // 重置所有通知状态（当重新加载任务时调用）
  const resetNotifications = () => {
    notifiedTasks.value.clear();
  };

  // 组件挂载时启动检查
  onMounted(() => {
    startPeriodicCheck();
  });

  // 组件卸载时清理
  onUnmounted(() => {
    stopPeriodicCheck();
  });

  return {
    isEnabled,
    sendNotification,
    checkOverdueTasks,
    checkUpcomingTasks,
    notifyTaskCompleted,
    startPeriodicCheck,
    stopPeriodicCheck,
    clearTaskNotification,
    resetNotifications,
  };
}