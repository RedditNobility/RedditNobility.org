import { BasicResponse } from "../Response";
import { Properties } from "./User";
import http from "@/http-common";
export enum Level {
    Moderator,
    Recruiter,
    Retired,
}
export interface TeamMember {
    id: number;
    user: TeamUser;
    level: Level;
    description: string;
    created: number;
}
export interface TeamInfo {
    user: string;
    level: string;
    description: string;
}

export interface TeamUser {
    id: number;
    username: string;
    properties: Properties;
}

export interface MapResponse {
    Moderator: TeamMember[];
    Retired:   TeamMember[];
    Recruiter: TeamMember[];
}


export async function getTeamList(): Promise<Array<TeamMember> | undefined> {
    //${API_URL}
    const value = await http.get("/team/get/list",);

    if (value.status != 200) {
        return undefined;
    }
    const data = value.data as BasicResponse<unknown>;
    if (data.success) {
        return data.data as Array<TeamMember>;
    }

    return undefined;
}
export async function getTeamMap(): Promise<MapResponse | undefined> {
    //${API_URL}
    const value = await http.get("/team/get");

    if (value.status != 200) {
        return undefined;
    }
    const data = value.data as BasicResponse<MapResponse>;
    if (data.success) {
        return data.data;
    }

    return undefined;
}