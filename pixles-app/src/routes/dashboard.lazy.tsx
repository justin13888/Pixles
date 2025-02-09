import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card"
import { Table, TableHeader, TableRow, TableHead, TableBody, TableCell } from "@/components/ui/table"
import { filesize } from 'filesize'
import { capitalCase } from "change-case";

import { useQuery } from '@tanstack/react-query'
import { createLazyFileRoute } from '@tanstack/react-router'
import { Album, HardDrive, Image, type LucideIcon, } from "lucide-react"
import { activityToDescription } from "@/lib/formatter"

export const Route = createLazyFileRoute('/dashboard')({
  component: () => <Dashboard />
})

const Dashboard = () => {
  return <div>Dashboard</div>;
  // const {
  //   data: statsData,
  //   isLoading: isStatsLoading,
  //   isError: isStatsError
  // } = useQuery({
  //   queryKey: ['stats'],
  //   // queryFn: async () => api.v1.dashboard.stats.get(),
  //   queryFn: async () => ({})
  // });

  // const {
  //   data: recentActivityData,
  //   isLoading: isRecentActivityLoading,
  //   isError: isRecentActivityError,
  // } = useQuery({
  //   queryKey: ['recentActivity'],
  //   // queryFn: async () => api.v1.dashboard['recent-activity'].get(),
  //   queryFn: async () => ({})
  // });

  // return (
  //   // <div>
  //   //   <h1>Dashboard</h1>
  //   //   <div>
  //   //     <h2>Stats</h2>
  //   //     <pre>{statsQuery.data && JSON.stringify(statsQuery.data, null, 2)}</pre>
  //   //   </div>
  //   //   <div>
  //   //     <h2>Recent Activity</h2>
  //   //     <pre>{recentActivityQuery.data && JSON.stringify(recentActivityQuery.data, null, 2)}</pre>
  //   //   </div>
  //   // </div>
  //   <div className="flex flex-col w-full min-h-screen bg-background">
  //     <main className="flex flex-col gap-8 p-4 md:p-10">
  //       <h1 className="text-3xl font-bold">Dashboard</h1>
  //       <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
  //         {(() => {
  //           // totalPhotos: 100,
  //           // totalAlbums: 10,
  //           // storageUsed: 1000000,

  //           let content: { title: string, icon: LucideIcon, value: string, subValue?: string }[];
  //           if (isStatsLoading) {
  //             content = [
  //               { title: 'Photos', icon: Image, value: 'Loading...' },
  //               { title: 'Albums', icon: Album, value: 'Loading...' },
  //               { title: 'Storage Used', icon: HardDrive, value: 'Loading...' },
  //             ]
  //           } else if (isStatsError) {
  //             content = [
  //               { title: 'Photos', icon: Image, value: 'Error' },
  //               { title: 'Albums', icon: Album, value: 'Error' },
  //               { title: 'Storage Used', icon: HardDrive, value: 'Error' },
  //             ]
  //           } else if (!statsData || !statsData.data) {
  //             content = [
  //               { title: 'Photos', icon: Image, value: 'No data' },
  //               { title: 'Albums', icon: Album, value: 'No data' },
  //               { title: 'Storage Used', icon: HardDrive, value: 'No data' },
  //             ]
  //           } else {
  //             content = [
  //               { title: 'Photos', icon: Image, value: statsData.data.totalPhotos.toLocaleString() },
  //               { title: 'Albums', icon: Album, value: statsData.data.totalAlbums.toLocaleString() },
  //               {
  //                 title: 'Storage Used', icon: HardDrive, value: filesize(
  //                   statsData.data.storageUsed, {
  //                   base: 2,
  //                   standard: 'jedec',
  //                 }
  //                 )
  //               },
  //             ]
  //           }

  //           return (
  //             content.map((item) => {
  //               return (
  //                 <Card key={item.title}>
  //                   <CardHeader className="flex flex-row items-center justify-between pb-2">
  //                     <CardTitle className="text-sm font-medium">{item.title}</CardTitle>
  //                     <item.icon className="w-4 h-4 text-muted-foreground" />
  //                   </CardHeader>
  //                   <CardContent>
  //                     <div className="text-2xl font-bold">{item.value}</div>
  //                     <p className="text-xs text-muted-foreground">{item.subValue || ""}</p>
  //                   </CardContent>
  //                 </Card>
  //               )
  //             })
  //           )
  //         })()}
  //       </div>

  //       <h2 className="text-2xl">Stats </h2>
  //       <Card>
  //         <Table>
  //           <TableHeader>
  //             <TableRow>
  //               <TableHead className="w-[200px]">Date</TableHead>
  //               <TableHead className="text-center">Type</TableHead>
  //               <TableHead className="text-center">Action</TableHead>
  //               <TableHead>Description</TableHead>
  //             </TableRow>
  //           </TableHeader>
  //           <TableBody>
  //             {
  //               isRecentActivityLoading ? (
  //                 <TableRow>
  //                   <TableCell colSpan={4} className="text-center">Loading...</TableCell>
  //                 </TableRow>
  //               ) : isRecentActivityError ? (
  //                 <TableRow>
  //                   <TableCell colSpan={4} className="text-center">Error</TableCell>
  //                 </TableRow>
  //               ) : !recentActivityData || !recentActivityData.data ? (
  //                 <TableRow>
  //                   <TableCell colSpan={4} className="text-center">No data</TableCell>
  //                 </TableRow>
  //               ) : (
  //                 recentActivityData.data.map((activity) => {
  //                   return (
  //                     <TableRow key={`${activity.date}${activity.type}`}>
  //                       <TableCell className="font-bold">{activity.date.toLocaleString()}</TableCell>
  //                       <TableCell className="text-center">{capitalCase(activity.type)}</TableCell>
  //                       <TableCell className="text-center">{capitalCase(activity.action)}</TableCell>
  //                       <TableCell>{activityToDescription(activity)}</TableCell>
  //                     </TableRow>
  //                   )
  //                 })
  //               )
  //             }
  //           </TableBody>
  //         </Table>
  //       </Card>
  //     </main>
  //   </div>
  // )
}
// TODO: Tweak aesthetics a little bit more
