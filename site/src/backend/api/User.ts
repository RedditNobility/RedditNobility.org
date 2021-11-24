import { BasicResponse } from "../Response";
import http from "@/http-common";
export interface Content {
  url: string | undefined;
  content: string | undefined;
  over_18: boolean;
}export interface RedditPost {
  subreddit: string;
  url: string;
  id: string;
  title: string;
  content: Content;
  score: number;

}export interface Comment {
  subreddit: string;
  url: string;
  id: string;
  og_post_title: string;
  content: Content;
  score: number;

}
export interface RedditUser {
  name: string;
  avatar: string;
  comment_karma: number;
  total_karma: number;
  created: number;
  top_posts: Array<RedditPost>
  top_comments: Array<Comment>
  user: User;
}export interface User {
  id: number;
  discord_id: number;
  username: string;
  permissions: UserPermissions;
  status: string;
  status_changed: number;
  moderator: string;
  discoverer: string;
  properties: Properties;
  created: number;
}
export interface MeResponse {
  id: number;
  username: string;
  permissions: UserPermissions;
  created: number;
}
export interface Properties {
  title: string | undefined;
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
} export async function reviewUser(token: string): Promise<RedditUser | undefined> {
  //${API_URL}
  const value = await http.get("/api/moderator/review/next", {
    headers: {
      Authorization: "Bearer " + token,
    },
  });

  if (value.status != 200) {
    return undefined;
  }
  const data = value.data as BasicResponse<unknown>;
  if (data.success) {
    return data.data as RedditUser;
  }

  return undefined;
} export async function reviewUserByName(token: string, username: string): Promise<RedditUser | undefined> {
  //${API_URL}
  const value = await http.get("/api/moderator/review/"+username, {
    headers: {
      Authorization: "Bearer " + token,
    },
  });

  if (value.status != 200) {
    return undefined;
  }
  const data = value.data as BasicResponse<unknown>;
  if (data.success) {
    return data.data as RedditUser;
  }

  return undefined;
}