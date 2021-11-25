import { BasicResponse } from "@/backend/Response";
import http from "@/http-common";
import { UserStats } from "./api/User";
export async function getTitles() {

    console.log("Getting Titles");
    return await http
        .get("/titles")
        .then((res) => {
            console.log(typeof res);
            if (res.status != 200) {
                return;
            }
            const result = res.data;
            let value = JSON.stringify(result);
            let response: BasicResponse<unknown> = JSON.parse(value);
            const data = response as BasicResponse<Array<string>>;
            return data.data;
        })
        .catch((error) => {
            console.error(error);
        });

} 
export async function getSystemStats(token: string): Promise<UserStats | undefined> {
    //${API_URL}
    const value = await http.get("/moderator/stats", {
      headers: {
        Authorization: "Bearer " + token,
      },
    });
  
    if (value.status != 200) {
      return undefined;
    }
    const data = value.data as BasicResponse<unknown>;
    if (data.success) {
      return data.data as UserStats;
    }
  
    return undefined;
  }