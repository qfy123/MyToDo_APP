// 前端 TypeScript 类型定义
export interface Task {
  id?: number;
  title: string;
  description: string;
  due_date?: string;
  priority: TaskPriority;
  is_completed: boolean;
  created_at: string;
  tags: string[]; // 任务标签列表
  sort_order: number; // 新增：排序顺序
}

export interface Tag {
  id?: number;
  name: string;
  color: string; // 标签颜色（十六进制）
  created_at: string;
}

export enum TaskPriority {
  Low = 0,
  Medium = 1,
  High = 2,
}

export interface CreateTaskRequest {
  title: string;
  description?: string;
  due_date?: string;
  priority: TaskPriority;
  tags?: string[]; // 新增：创建任务时的标签
}

export interface UpdateTaskRequest {
  id: number;
  title?: string;
  description?: string;
  due_date?: string;
  priority?: TaskPriority;
  is_completed?: boolean;
  tags?: string[]; // 新增：更新任务时的标签
}

export interface CreateTagRequest {
  name: string;
  color: string;
}

export interface UpdateTagRequest {
  id: number;
  name?: string;
  color?: string;
}