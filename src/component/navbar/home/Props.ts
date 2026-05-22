import {HomeNavigation} from "../../../structures/navbar/home/homeNavBar.ts";

export interface Props {
    navigation: HomeNavigation,
    changeNavigation: (navigation: HomeNavigation) => void,
}