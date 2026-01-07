export const colorScheme = (() => {
    const dE = document.documentElement;
    const dcs = 'data-bs-theme';
    const pcs = 'prefers-color-scheme';

    const init = (): void => {
        if (window.matchMedia) {
            const prefersDark = window.matchMedia(`(${pcs}: dark)`);

            prefersDark.addEventListener('change', (e: MediaQueryListEvent) => {
                dE.setAttribute(dcs, e.matches ? 'dark' : 'light');
                updateTheme();
            });
        }
    };

    const updateTheme = () => {
        setAllScenes();
        updateSvgFilter();
    };

    // Inside colorScheme IIFE
    const getCurrentSeasonImage = (): 'valley' | 'tropical' | 'forest' | 'snow' => {
        const month = new Date().getMonth() + 1; // getMonth() is 0-indexed, so add 1

        if (month >= 3 && month <= 5) {
            return 'valley'; // March, April, May -- spring
        } else if (month >= 6 && month <= 8) {
            return 'tropical'; // June, July, August -- summer
        } else if (month >= 9 && month <= 11) {
            return 'forest'; // September, October, November -- autumn
        } else {
            return 'snow'; // December, January, February -- winter
        }
    };

    const setAllScenes = (): void => {
        const seasonImage = getCurrentSeasonImage();
        document.querySelectorAll<HTMLImageElement>('[data-variant]').forEach((element) => {
            if (element.getAttribute('data-variant') === 'image-light-dark') {
                const scheme = getScheme();
                const themeSuffix = scheme === 'light' ? 'day' : 'night';
                element.src = `/assets/scenes/${seasonImage}-${themeSuffix}.png`;
                console.log(element.src);
            }
        });
    };

    function updateSvgFilter() {
        const reactLightDarkElements = document.querySelectorAll('.react-light-dark');
        reactLightDarkElements.forEach((element) => {
            // Use the cached elements
            (element as HTMLImageElement).style.filter = window.matchMedia(`(${pcs}: dark)`).matches ? 'brightness(0.7) contrast(1.1)' : 'brightness(1) contrast(1)';
        });
    }

    // Method to check the initialization state
    const getScheme = (): 'light' | 'dark' | null => {
        return dE.getAttribute(dcs) as 'light' | 'dark' | null;
    };

    // Expose methods (init, getScheme) to be used outside
    return {
        init,
        getScheme,
        updateTheme,
    };
})();

// Automatically invoke init on import
colorScheme.init();
