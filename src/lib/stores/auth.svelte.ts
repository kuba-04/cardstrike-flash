export function createAuthStore() {
    let isLoggedIn = $state(false);
    let user = $state<{ name: string } | null>(null);

    function login(username: string) {
        isLoggedIn = true;
        user = { name: username };
    }

    function logout() {
        isLoggedIn = false;
        user = null;
    }

    return {
        get isLoggedIn() { return isLoggedIn },
        get user() { return user },
        login,
        logout
    };
}

export const auth = createAuthStore();
