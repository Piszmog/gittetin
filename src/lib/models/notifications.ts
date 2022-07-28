export interface Notification {
  id: string;
  title: string;
  url?: string;
  reason: string;
  unread: boolean;
  updated_at: string;
  last_read_at?: string;
  notification_type: string;
  repository: Repository;
}

export interface Repository {
  id: string;
  name: string;
  full_name: string;
  url: string;
}
