import {useState} from "react";
import {Navigation} from "../../structures/navigation/Navigation.ts";

export const useNavigation = () => {

    const [navigation, setNavigation] = useState<Navigation>(Navigation.Sounds);

    function changeNavigation(navigation: Navigation) {
        setNavigation(navigation)
    }

    return {
        navigation,
        changeNavigation
    }

};