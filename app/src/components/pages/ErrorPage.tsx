import { ErrorResponse, useRouteError } from "react-router-dom";

export default function ErrorPage() {
    const error = useRouteError() as ErrorResponse;
    console.error(error);

    return (
        <div id="error-page">
            <h1>Oops!</h1>
            <p>Sorry, an unexpected error has occurred.</p>
            <h1>{error?.status}</h1>
            <h2>({error?.statusText})</h2>
            <br />
            <br />
            <b className="">
                Server response:
                <br />
            </b>
            <em>{error?.data}</em>
        </div>
    );
}