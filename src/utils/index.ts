export function formatError(err: any): string {
  if (err.response && err.response.data && err.response.data.error) {
    return err.response.data.error;
  }
  return err.message || 'An unexpected error occurred';
}
