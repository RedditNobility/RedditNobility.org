import { BasicResponse, DEFAULT_USER_LIST } from "../Response";
import http from "@/http-common";

export interface User {
  id: number;
  discord_id: number;
  username: string;
  permissions: UserPermissions;
  status: string;
  status_changed: number;
  moderator: string;
  discoverer: string;
  created: number;
}
export interface MeResponse {
  id: number;
  username: string;
  permissions: UserPermissions;
  created: number;
}

export interface UserPermissions {
  admin: boolean;
  moderator: boolean;
  submit: boolean;
  approve_user: boolean;
  login: boolean;
}
export async function getUser(token: string) {
  //${API_URL}
  const value = await http.get("/api/me", {
    headers: {
      Authorization: "Bearer " + token,
    },
  });

  if (value.status != 200) {
    return null;
  }
  const data = value.data as BasicResponse<unknown>;
  if (data.success) {
    return data.data as User;
  }

  return null;
}