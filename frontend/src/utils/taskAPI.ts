import { invoke } from '@tauri-apps/api/core';
import type { Task, CreateTaskRequest, UpdateTaskRequest, Tag, CreateTagRequest, UpdateTagRequest } from '../types/task';

export class TaskAPI {
  static async createTask(request: CreateTaskRequest): Promise<Task> {
    return await invoke('create_task', { request });
  }

  static async getAllTasks(): Promise<Task[]> {
    return await invoke('get_all_tasks');
  }

  static async getTodayTasks(): Promise<Task[]> {
    return await invoke('get_today_tasks');
  }

  static async getCompletedTasks(): Promise<Task[]> {
    return await invoke('get_completed_tasks');
  }

  static async updateTask(request: UpdateTaskRequest): Promise<Task> {
    return await invoke('update_task', { request });
  }

  static async deleteTask(id: number): Promise<void> {
    return await invoke('delete_task', { id });
  }

  static async toggleTaskCompletion(id: number): Promise<Task> {
    return await invoke('toggle_task_completion', { id });
  }

  static async exportTasksToJson(): Promise<string> {
    return await invoke('export_tasks_to_json');
  }

  static async importTasksFromJson(jsonData: string): Promise<number> {
    return await invoke('import_tasks_from_json', { jsonData });
  }

  static async clearAllTasks(): Promise<void> {
    return await invoke('clear_all_tasks');
  }

  // === 标签相关方法 ===

  static async createTag(request: CreateTagRequest): Promise<Tag> {
    return await invoke('create_tag', { request });
  }

  static async getAllTags(): Promise<Tag[]> {
    return await invoke('get_all_tags');
  }

  static async updateTag(request: UpdateTagRequest): Promise<Tag> {
    return await invoke('update_tag', { request });
  }

  static async deleteTag(id: number): Promise<void> {
    return await invoke('delete_tag', { id });
  }

  // === 设置相关方法 ===

  static async getAppSettings(): Promise<any> {
    return await invoke('get_app_settings');
  }

  static async updateAppSettings(request: any): Promise<any> {
    return await invoke('update_app_settings', { request });
  }

  // === 窗口管理方法 ===

  static async handleCloseRequest(): Promise<string> {
    return await invoke('handle_close_request');
  }

  static async forceExitApp(): Promise<void> {
    return await invoke('force_exit_app');
  }

  static async minimizeToTray(): Promise<void> {
    return await invoke('minimize_to_tray');
  }

  // === 通知相关方法 ===

  static async sendNotification(title: string, body: string): Promise<void> {
    return await invoke('send_notification', { title, body });
  }

  static async checkOverdueTasks(): Promise<Task[]> {
    return await invoke('check_overdue_tasks');
  }

  static async getUpcomingTasks(minutesAhead: number): Promise<Task[]> {
    return await invoke('get_upcoming_tasks', { minutesAhead });
  }

}