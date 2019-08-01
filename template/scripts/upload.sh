if [ -z $S3_BUCKET ]; then
  echo "***** S3_BUCKET is required"
  echo "Usage: S3_BUCKET=my_bucket npm run upload"
  exit 1
fi

aws s3 sync ./pkg s3://$S3_BUCKET/ \
  --cache-control max-age=0,no-cache --delete --acl public-read

WASM_FILE=$(ls pkg/ | grep '.wasm$');
brotli-cli pkg/*.wasm
BROTLI_FILE=$(ls pkg/ | grep wasm.br);
mv pkg/$BROTLI_FILE pkg/$WASM_FILE

aws s3 cp pkg/*.wasm s3://$S3_BUCKET/ \
  --cache-control max-age=0,no-cache \
  --acl public-read \
  --content-type application/wasm \
  --content-encoding br

if [ ! -z $DISTRIBUTION_ID ]
then
  aws cloudfront create-invalidation --distribution-id $DISTRIBUTION_ID --paths '/*'
fi