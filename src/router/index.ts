import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
    {
        path: "/",
        component: () => import("../views/Home.vue")
    },
    {
        path: "/webSsh",
        component: () => import("../views/webssh/WebSsh.vue")
    },
    {
        name: "onlineTool",
        path: '/onlineTool',
        component: () => import("../views/tool/OnlineTool.vue"),
        redirect: '/onlineTool/pwdGenerate',
        children: [
            {
                path: 'pwdGenerate',
                component: () => import("../views/tool/PwdGenerate.vue")
            },
            {
                path: 'json',
                component: () => import("../views/tool/Json.vue")
            }
        ]
    },
];

const router = createRouter({
    history: createWebHashHistory(),
    routes
});

export default router;