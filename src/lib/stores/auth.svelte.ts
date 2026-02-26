export function createAuthStore() {
    let isLoggedIn = $state(false);
    let user = $state<{ email: string, password: string } | null>(null);

    function login(email: string, password: string) {
        isLoggedIn = true;
        user = { email, password };
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
