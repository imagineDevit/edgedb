# UpdateQuery

#### List of possibles attributes :

<table>
    <thead>
        <tr>
            <th rowspan="2">Attributes</th>
            <th rowspan="2">Optional</th>
            <th rowspan="2">Description</th>
            <th colspan="3">Options</th>
        </tr>
        <tr>
            <th>Name</th>
            <th>Optional</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td rowspan=2> <strong style="color: #008200">meta</strong> </td>
            <td rowspan=2>No</td>
            <td rowspan=2>
                The field decorated <i style="color: #91b362">#[meta]</i> represents the query metadata 
                and is used just to declare module and table names.
                This field must be (called __meta__, of type () ) : <br>
                <strong style="color: #c82829">__meta__ : ()</strong>
            </td>
            <td><i style="color: yellow">module</i></td>
            <td>Yes ("default") </td>
            <td>The edgedb module name </td>
        </tr>
        <tr>
            <td><i style="color: yellow">table</i></td>
            <td>No</td>
            <td>The edgedb table name </td>
        </tr>
        <tr>
            <td> <strong style="color: #008200">param</strong> </td>
            <td> Yes </td>
            <td colspan="4"> 
            The <strong style="color: #91b362">param</strong> attribute represents the query parameter label associated to the decorated field. </td>
        </tr>
         <tr>
            <td> <strong style="color: #008200">filters</strong> </td>
            <td>Yes</td>
            <td colspan="4"><i style="color: #91b362">filters</i> attribute is used to combine all the filters in a unique annotated <strong style="color: #9d00ec">#[derive(EdgedbFilters)]</strong> <br> see <a href="../shape-macros/edgedb-filters.html"> EdgedbFilters</a></td>
        </tr>
    </tbody>
</table>
<br>
