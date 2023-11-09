type PropertyKey = string | number;

export type CreateCollectionData = {
    mode: string, 
    name?: string, 
    description?: string, 
    tokenPrefix?: string,
    mutableProperties?: PropertyKey[],
    properties?: any[],
}

export function get_create_collection_data(data: CreateCollectionData) {
    const name = data.name || 'CollectionName';
    const description = data.description || 'CollectionDescription';
    const tokenPrefix = data.tokenPrefix || 'STP';
    const propertyPermissions = data.mutableProperties?.map(key => {
        return {
            key: key,
            permission: {
                mutable: true
            }
        };
    });

    return {
        mode: data.mode,
        name: stringToUnicodeArr(name),
        description: stringToUnicodeArr(description),
        tokenPrefix: stringToUnicodeArr(tokenPrefix),
        propertyPermissions: propertyPermissions ?? [],
        properties: data.properties ?? [],
    }
}

export function stringToUnicodeArr(str: string): number[] {
    return str.split('').map(c => c.charCodeAt(0));
}