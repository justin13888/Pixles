import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { filesize } from "filesize";
import { toast } from "sonner"

import { useQuery } from "urql";
import { createLazyFileRoute, Link } from "@tanstack/react-router";
import { Album, HardDrive, Image, Loader2, RotateCw, type LucideIcon } from "lucide-react";

export const Route = createLazyFileRoute("/dashboard")({
  component: () => <Dashboard />,
});

import { graphql, useFragment } from "@/gql";
import { formatDate } from "@/lib/formatter";
import { useEffect, type JSX } from "react";

const DashboardQuery = graphql(`
  query DashboardQuery {
    activity {
      search {
        ...RecentActivityFragment
      }
    }
    user {
      statistics {
        totalPhotos
        totalAlbums
        usedStorage
      }
    }
  }
`);

const RecentActivityFragment = graphql(`
  fragment RecentActivityFragment on Activity {
    __typename
    id
    type
    action
    timestamp
    ... on CreateAlbumActivity {
      albumId
      albumName
      userId
    }
    ... on DeleteAlbumActivity {
      albumId
      albumName
      userId
    }
    ... on UpdateAlbumActivity {
      albumId
      oldName
      newName
      userId
      changes
    }
    ... on UploadAssetsActivity {
      destinationAlbumId
      destinationAlbumName
      assetCount
      assetTotalSize
    }
    ... on DeleteAssetActivity {
      assetId
      assetName
      sourceAlbumId
      userId
    }
    ... on MoveAssetActivity {
      assetId
      assetName
      userId
      sourceAlbumId
      sourceAlbumName
      targetAlbumId
      targetAlbumName
      userId
    }
  }
`);

const Dashboard = () => {
  const [{ data, fetching, error }, reexecuteQuery] = useQuery({
    query: DashboardQuery,
    // variables:
  });
  const activitieRefs = data?.activity.search;
  const stats = data?.user.statistics;

  // return (
  //   <div>
  //     <h1>Dashboard</h1>
  //     <pre>{JSON.stringify(data, null, 2)}</pre>
  //   </div>
  // );

  return (
    <div className="flex flex-col w-full min-h-screen bg-background">
      <main className="flex flex-col gap-8 p-4 md:p-10">
        <div className="flex items-center justify-between gap-4">
          <h1 className="text-3xl font-bold">Dashboard</h1>
          <button
            type="button"
            onClick={async () => {
              const toastId = toast.loading('Refetching...');
              reexecuteQuery();

              while (fetching) {
                await new Promise(resolve => setTimeout(resolve, 100));
              }

              if (error) {
                toast.error('Error fetching data', { id: toastId });
              } else {
                toast.success('Data fetched successfully', { id: toastId, duration: 800 });
              }
            }}
            className="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 w-10"
          >
            {fetching ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <RotateCw className="h-4 w-4" />
            )}
          </button>
        </div>
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
          {(() => {
            // TODO: Make buttons clickable

            let content: { title: string, icon: LucideIcon, value: string, subValue?: string, link?: string }[];
            if (fetching) {
              content = [
                { title: 'Photos', icon: Image, value: 'Loading...' },
                { title: 'Albums', icon: Album, value: 'Loading...' },
                { title: 'Storage Used', icon: HardDrive, value: 'Loading...' },
              ]
            } else if (error) {
              content = [
                { title: 'Photos', icon: Image, value: 'Error' },
                { title: 'Albums', icon: Album, value: 'Error' },
                { title: 'Storage Used', icon: HardDrive, value: 'Error' },
              ]
            } else if (!stats) {
              content = [
                { title: 'Photos', icon: Image, value: 'No data', link: '/albums' },
                { title: 'Albums', icon: Album, value: 'No data', link: '/albums' },
                { title: 'Storage Used', icon: HardDrive, value: 'No data', link: '/storage' },
              ]
            } else {
              content = [
                { title: 'Photos', icon: Image, value: stats.totalPhotos.toLocaleString(), link: '/albums' },
                { title: 'Albums', icon: Album, value: stats.totalAlbums.toLocaleString(), link: '/albums' },
                {
                  title: 'Storage Used',
                  icon: HardDrive,
                  value: filesize(
                    stats.usedStorage,
                    {
                      base: 2,
                      standard: 'jedec',
                    }
                  ),
                  link: '/storage',
                },
              ]
            }

            return (
              content.map((item) => {
                return (
                  <Link key={item.title} to={item.link || ""} disabled={!item.link}>
                    <Card className="transition-colors hover:bg-muted/50">
                      <CardHeader className="flex flex-row items-center justify-between pb-2">
                        <CardTitle className="text-sm font-medium">{item.title}</CardTitle>
                        <item.icon className="w-4 h-4 text-muted-foreground" />
                      </CardHeader>
                      <CardContent>
                        <div className="text-2xl font-bold">{item.value}</div>
                        <p className="text-xs text-muted-foreground">{item.subValue || ""}</p>
                      </CardContent>
                    </Card>
                  </Link>
                )
              })
            )
          })()}
        </div>

        <h2 className="text-2xl">Stats </h2>
        <Card>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-[200px]">Timestamp</TableHead>
                <TableHead>Activity</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {
                fetching ? (
                  <TableRow>
                    <TableCell colSpan={4} className="text-center">Loading...</TableCell>
                  </TableRow>
                ) : error ? (
                  <TableRow>
                    <TableCell colSpan={4} className="text-center">Error</TableCell>
                  </TableRow>
                ) : !activitieRefs ? (
                  <TableRow>
                    <TableCell colSpan={4} className="text-center">No data</TableCell>
                  </TableRow>
                ) : (
                  activitieRefs.map((ref) => {
                    // console.log('dfdf', ref)
                    const data = useFragment(RecentActivityFragment, ref);
                    // console.log('fdfd', data)
                    return (
                      <TableRow key={data.id}>
                        <TableCell className="font-bold">{formatDate(new Date(data.timestamp))}</TableCell>
                        <TableCell>
                          {((): JSX.Element => {
                            // TODO: Make links more obvious by styling
                            switch (data.__typename) {
                              case 'CreateAlbumActivity':
                                return (
                                  <p>
                                    <span className="font-bold">Created album</span>
                                    {" "}
                                    <a href={`/albums/${data.albumId}`} className="text-muted-foreground">
                                      {data.albumName}
                                    </a>
                                  </p>
                                )
                              case 'DeleteAlbumActivity':
                                return (
                                  <p>
                                    <span className="font-bold">Deleted album</span>
                                    {" "}
                                    <a href={`/albums/${data.albumId}`} className="text-muted-foreground">
                                      {data.albumName}
                                    </a>
                                  </p>
                                )
                              case 'UpdateAlbumActivity':
                                return (
                                  <p>
                                    <span className="font-bold">Updated album</span>
                                    {" "}
                                    from
                                    {" "}
                                    <span className="text-muted-foreground">
                                      {data.oldName}
                                    </span>
                                    {" "}
                                    to
                                    {" "}
                                    <a href={`/albums/${data.albumId}`} className="text-muted-foreground">
                                      {data.newName}
                                    </a>
                                  </p>
                                )
                              case 'UploadAssetsActivity':
                                return (
                                  <p>
                                    <span className="font-bold">Uploaded assets</span>
                                    {" "}
                                    ({data.assetCount} qty., {filesize(data.assetTotalSize, { base: 2, standard: 'jedec' })})
                                    {" "}
                                    to
                                    {" "}
                                    <a href={`/albums/${data.destinationAlbumId}`} className="text-muted-foreground">
                                      {data.destinationAlbumName}
                                    </a>
                                  </p>
                                )
                              case 'DeleteAssetActivity':
                                return (
                                  <p>
                                    <span className="font-bold">Deleted asset</span>
                                    {" "}
                                    {data.assetName}
                                  </p>
                                )
                              case 'MoveAssetActivity':
                                return (
                                  <p>
                                    <span className="font-bold">Moved asset</span>
                                    {" "}
                                  </p>
                                )
                            }
                          })()}
                        </TableCell>
                      </TableRow>
                    )
                  })
                )
              }
            </TableBody>
          </Table>
        </Card>
      </main>
    </div>
  )
};
