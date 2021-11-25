import { BasicResponse } from "@/backend/Response";
import http from "@/http-common";
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