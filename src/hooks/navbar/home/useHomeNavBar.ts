import {useState} from "react";
import {HomeNavigation} from "../../../structures/navbar/home/homeNavBar.ts";

export const useHomeNavBar = () => {

    const [navigation, setNavigation] = useState<HomeNavigation>(HomeNavigation.Sounds);

    function changeNavigation(navigation: HomeNavigation) {
        setNavigation(navigation)
    }

    return {
        navigation,
        changeNavigation
    }

};