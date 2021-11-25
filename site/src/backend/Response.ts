export interface BasicResponse<T> {
  success: boolean;
  data: T;
  status_code: number;
}
export interface APIError {
  user_friendly_message: string;
  error_code: string;
}

export interface LoginRequest {
  auth_token: AuthToken;
}
export interface AuthToken {
  id: number;
  user: number;
  token: string;
  expiration: number;
  created: number;
}
export interface SettingReport {
  email: EmailSettings;
  general: GeneralSetting;
}

export interface EmailSettings {
  email_username: DBSetting;
  email_host: DBSetting;
  encryption: DBSetting;
  from: DBSetting;
  port: DBSetting;
}

export interface DBSetting {
  id: number;
  setting: Setting;
  value: string;
  updated: number;
}

export interface Setting {
  key: string;
  name: string;
  default: null | string;
  optional: null;
  properties: null;
  options: string[] | null;
  public: boolean | null;
}

export interface GeneralSetting {
  name: DBSetting;
  installed: DBSetting;
  version: DBSetting;
}

export interface FileResponse {
  name: string;
  full_path: string;
  directory: boolean;
  data: Map<string, any>;
}

export interface Version {
  version: string;
  artifacts: string[];
}

export interface RepoSummary {
  name: string;
  storage: string;
  page_provider: string;
  repo_type: string;
  visibility: string;
}

export interface Project {
  repo_summary: RepoSummary;
  versions: Version[];
  frontend_response: null;
}
