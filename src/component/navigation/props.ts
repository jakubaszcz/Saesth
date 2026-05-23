import {Navigation} from "../../structures/navigation/Navigation.ts";

export interface Props {
    navigation: Navigation,
    changeNavigation: (navigation: Navigation) => void,
}