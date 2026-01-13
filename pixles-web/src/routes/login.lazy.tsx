import { Button } from '@/components/ui/button';
import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Link, createLazyFileRoute } from '@tanstack/react-router';
import { MountainIcon } from 'lucide-react';

export const Route = createLazyFileRoute('/login')({
    component: Login,
});

function Login() {
    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-muted/40 p-4">
            <Link to="/" className="mb-8 flex items-center gap-2">
                <MountainIcon className="h-8 w-8 text-primary" />
                <span className="text-2xl font-bold text-primary">Pixles</span>
            </Link>
            <Card className="w-full max-w-sm">
                <CardHeader>
                    <CardTitle className="text-2xl">Login</CardTitle>
                    <CardDescription>
                        Enter your email below to login to your account.
                    </CardDescription>
                </CardHeader>
                <CardContent className="grid gap-4">
                    <div className="grid gap-2">
                        <label htmlFor="email">Email</label>
                        <Input
                            id="email"
                            type="email"
                            placeholder="m@example.com"
                            required
                        />
                    </div>
                    <div className="grid gap-2">
                        <label htmlFor="password">Password</label>
                        <Input id="password" type="password" required />
                    </div>
                </CardContent>
                <CardFooter className="flex flex-col gap-4">
                    <Link to="/photos" className="w-full">
                        <Button className="w-full">Sign in</Button>
                    </Link>
                    <p className="text-xs text-muted-foreground text-center">
                        Don't have an account?{' '}
                        <span className="underline cursor-pointer">
                            Sign up
                        </span>
                    </p>
                </CardFooter>
            </Card>
        </div>
    );
}
