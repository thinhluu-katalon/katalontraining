<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Welcome ramya                          _f92d6d</name>
   <tag></tag>
   <elementGuidId>50fb4c56-a111-42c6-a053-c5439064a850</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#wrapper</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='wrapper']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>16a0034d-afd7-406c-9bf1-33da77267174</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>wrapper</value>
      <webElementGuid>592b26af-ee7f-41bb-8e54-8fa7a80fb68e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

            
                
                Welcome ramya
                


    
        
        
    




    

        
        
            
                
                                    
                No new notifications
                
                                    
            
        
    

    
        
            
                
                                    
            

        

        
            


        

        
            

        
    




    
        
            

        
            
        

    



    
        
            Success!        
        
            Successfully Saved        
        
            Successfully Shared        
        
            Successfully Deleted        

    




    
        
            
        
    





    
        People who shared this post    
    
        
            
        

    





    
        People who like this post    
    
        
            
        

    




    var getAccessUrl = '/index.php/buzz/getLogedToBuzz';
    var loginpageURL = '/index.php/auth/login';

    // buzzCommon.js
    var viewLikedEmployees = '/index.php/buzz/viewLikedEmployees';
    var addBuzzCommentURL = '/index.php/buzz/addNewComment';
    var shareShareURL = '/index.php/buzz/shareAPost';
    var getLikedEmployeeListURL = '/index.php/buzz/getLikedEmployeeList';

    // buzzNew.js
    var shareLikeURL = '/index.php/buzz/likeOnShare';
    var commentLikeURL = '/index.php/buzz/likeOnComment';
    var getSharedEmployeeListURL = '/index.php/buzz/getSharedEmployeeList';

    // viewNotificationComponent.js
    var viewMoreShare = '/index.php/buzz/viewShare';
    var buzzURL = &quot;/index.php/buzz/viewBuzz&quot;;
    var ClearNotificationURL = &quot;/index.php/buzz/clearNotificationAjax&quot;;
    var ClickOnNotificationIconURL = &quot;/index.php/buzz/clickOnNotificationIconAjax&quot;;
    var lang_NoNewNotifications = 'No new notifications';
    var lang_NotificationClearFailed = 'Failed to clear notifications';

                

    
        
    

                                    
                        
                    
                                
                    var marketplaceURL = &quot;/index.php/marketPlace/ohrmAddons&quot;;
                    var SubscriberURL = &quot;/index.php/pim/subscriber&quot;;
                
                
                    
                        
    About

    
        
            ×
            About
        
        
            
                
                    
                        
                            Company Name: OrangeHRM
                        
                        
                            Version: Orangehrm OS 4.10.1
                        
                        
                            Active Employees: 96
                        
                        
                            Employees Terminated: 0
                        
                    
                
                
                        
    



    jQuery(document).ready(function() {
        
               
        jQuery('#aboutDisplayLink').click(function(event) {
            event.stopImmediatePropagation();
            jQuery('#messageToDisplayAbout').css(
                    'display', 'none');
            jQuery('#displayAbout').modal();
            jQuery('#help-menu.panelContainer').attr('style', 'display:block');
            
            var test = jQuery('.panelContainer');
            jQuery('#help-menu.panelContainer').css(
                    'display', 'block');
             
        });

        jQuery('#heartbeatSubmitBtn').click(function(event) {
            event.stopImmediatePropagation();
            jQuery(this).closest('form').ajaxSubmit(function() {
                jQuery('#messageToDisplayAbout').html('Saved');

                if (jQuery('#register_registration').is(':checked')) {
                    jQuery('#registration-section').css(
                            'display', 'none');
                }
                jQuery('#displayAbout').modal('hide');
                jQuery('#messageToDisplayAbout').css(
                        'display', 'block');
                jQuery('#welcome-menu').css('display','none');
            });
        });

        jQuery('#displayAbout').click(function(event) {
            event.stopImmediatePropagation();
        });
        
        jQuery('#heartbeatCancelBtn').click(function(event) {
            event.stopImmediatePropagation();
             jQuery('#welcome-menu').css('display','none');
                 jQuery('#displayAbout').modal('hide');
        });

    })



                        Support
                        Logout
                    
                
                                


    svg path,
    svg rect{
        fill: #FF6700;
    }
    .svgcl{
        position: relative;
        left: -35px;
        top: -31px;
    }
    


    var inputDatePattern = 'Y-m-d' ;
    var separatorString = 'to';
    $( document ).ready(function() {

        $(&quot;#loader-1&quot;).hide();
        empId = location.href[location.href.length-1];
        dates = $('#startDates').find(&quot;:selected&quot;).text().split(&quot; &quot;+separatorString+&quot; &quot;);
        startDate_timesheet = dates[0]+&quot; 00:00:00&quot;;
        endDate_timesheet   = dates[1]+&quot; 00:00:00&quot;;

        clientId  =     &quot;&quot;;
        clientSecret  = &quot;&quot;;
        clientUrl     = &quot;&quot;;
        successUrl  = &quot;&quot;;
        ajaxURL = &quot;/index.php/buzz/viewBuzz&quot;;
        var timeSheetStatus = $('#timesheet_status').find('h2').text();
        if(timeSheetStatus == 'Status: Approved'){

            $('.syncToggl').hide();
        } else {
            $('.syncToggl').show();
        }

    });

        
    function ajaxSyc() {
        $(&quot;#loader-1&quot;).show();

        $.ajax({
                type: &quot;POST&quot;,
                url: ajaxURL,
                data: {
                    'employee_Id':employeeId,
                    'startTime': startDate_timesheet,
                    'endTime': endDate_timesheet,
                    'timeFormat': inputDatePattern,
                    'timeZone': 'GMT'+formatTimeZone()
                },
                contentType: &quot;application/x-www-form-urlencoded&quot;,

                success: function (msg, status, jqXHR) {

                    $(&quot;#loader-1&quot;).hide();
                    msg = JSON.parse(msg);
                    msgCode = msg.statusCode;
                    if (msgCode != null) {
                        if (msgCode == 101) {
                            displayMessages('error',msg.description );
                        } else if (msgCode == 102) {

                            displayMessages('success', msg.description);
                            setTimeout(function () {
                                location.reload();
                            }, 2000);

                        }
                    } else {
                        showErrorMsg();
                    }

                },
                error: function (XMLHttpRequest, textStatus, errorThrown) {
                    $(&quot;#loader-1&quot;).hide();
                    console.log(errorThrown);
                    showErrorMsg();
                }
            });
    }
    
    function startSyc() {
        $(&quot;#loader-1&quot;).show();

    $.ajax({

        type: &quot;POST&quot;,
        url: clientUrl,


        data: {
            'grant_type': 'client_credentials',
            'client_id': clientId,
            'client_secret': clientSecret
        },
        contentType: &quot;application/x-www-form-urlencoded&quot;,


        success: function (msg, status, jqXHR) {

            try {

                msg = $.parseJSON(jqXHR.responseText);

            } catch (err) {
                console.log(err);
                showErrorMsg();
            }

            $.ajax({
                type: &quot;POST&quot;,
                url: successUrl,
                beforeSend: function (xhr) {

                    xhr.setRequestHeader(&quot;Authorization&quot;, &quot;Bearer &quot; + msg.access_token);
                },

                data: {

                    'employee_Id':employeeId,
                    'startTime': startDate_timesheet,
                    'endTime': endDate_timesheet,
                    'timeFormat': inputDatePattern,
                    'timeZone': 'GMT'+formatTimeZone()
                },
                contentType: &quot;application/x-www-form-urlencoded&quot;,

                success: function (msg, status, jqXHR) {

                    $(&quot;#loader-1&quot;).hide();
                    msgCode = msg.statusCode;
                    if (msgCode != null) {
                        if (msgCode == 101) {
                            displayMessages('error',msg.description );
                        } else if (msgCode == 102) {

                            displayMessages('success', msg.description);
                            setTimeout(function () {
                                location.reload();
                            }, 2000);

                        }
                    } else {
                        showErrorMsg();
                    }

                },
                error: function (XMLHttpRequest, textStatus, errorThrown) {
                    $(&quot;#loader-1&quot;).hide();
                    console.log(errorThrown);
                    showErrorMsg();
                }
            });

        },
        error: function (XMLHttpRequest, textStatus, errorThrown) {
            $(&quot;#loader-1&quot;).hide();
            console.log(errorThrown);
            showErrorMsg();
        }


    });

    }

    function showErrorMsg(){
        displayMessages('error','Unable To Sync With Toggl' );
        setTimeout(function () {
            $('#msgDiv').remove();
        }, 3000);

    }




    
    
        
            
  
      
  
  
        
    

    
        
            ×
            Confirm Toggl Sync
        
        
            Any existing timesheet entry will be overwritten if record for same date is matched. Click ok to continue.
        
        
                            
                        
        
    






             

            

    
    
        
        
            Admin

            
                        
                    
                    
                    User Management

                        
                            

                                
                                    Users

                                
                             

                        
                           
                    
                    
                    Job

                        
                            

                                
                                    Job Titles

                                
                                    Pay Grades

                                
                                    Employment Status

                                
                                    Job Categories

                                
                                    Work Shifts

                                
                             

                        
                           
                    
                    
                    Organization

                        
                            

                                
                                    General Information

                                
                                    Locations

                                
                                    Structure

                                
                             

                        
                           
                    
                    
                    Qualifications

                        
                            

                                
                                    Skills

                                
                                    Education

                                
                                    Licenses

                                
                                    Languages

                                
                                    Memberships

                                
                             

                        
                           
                    
                    
                    Nationalities

                        
                           
                    
                    
                    Corporate Branding

                        
                           
                    
                    
                    Configuration

                        
                            

                                
                                    Email Configuration

                                
                                    Email Subscriptions

                                
                                    Localization

                                
                                    Language Packages

                                
                                    Modules

                                
                                    Social Media Authentication

                                
                                    Register OAuth Client

                                
                             

                        
                           
                    
                                                
                                         
            
            
        
            PIM

            
                        
                    
                    
                    Configuration

                        
                            

                                
                                    Optional Fields

                                
                                    Custom Fields

                                
                                    Data Import

                                
                                    Reporting Methods

                                
                                    Termination Reasons

                                
                             

                        
                           
                    
                    
                    Employee List

                        
                           
                    
                    
                    Add Employee

                        
                           
                    
                    
                    Reports

                        
                           
                    
                                                
                                         
            
            
        
            Leave

            
                        
                    
                    
                    Apply

                        
                           
                    
                    
                    My Leave

                        
                           
                    
                    
                    Entitlements

                        
                            

                                
                                    Add Entitlements

                                
                                    Employee Entitlements

                                
                                    My Entitlements

                                
                             

                        
                           
                    
                    
                    Reports

                        
                            

                                
                                    Leave Entitlements and Usage Report

                                
                                    My Leave Entitlements and Usage Report

                                
                             

                        
                           
                    
                    
                    Configure

                        
                            

                                
                                    Leave Period

                                
                                    Leave Types

                                
                                    Work Week

                                
                                    Holidays

                                
                             

                        
                           
                    
                    
                    Leave List

                        
                           
                    
                    
                    Assign Leave

                        
                           
                    
                                                
                                         
            
            
        
            Time

            
                        
                    
                    
                    Timesheets

                        
                            

                                
                                    My Timesheets

                                
                                    Employee Timesheets

                                
                             

                        
                           
                    
                    
                    Attendance

                        
                            

                                
                                    My Records

                                
                                    Punch In/Out

                                
                                    Employee Records

                                
                                    Configuration

                                
                             

                        
                           
                    
                    
                    Reports

                        
                            

                                
                                    Project Reports

                                
                                    Employee Reports

                                
                                    Attendance Summary

                                
                             

                        
                           
                    
                    
                    Project Info

                        
                            

                                
                                    Customers

                                
                                    Projects

                                
                             

                        
                           
                    
                                                
                                         
            
            
        
            Recruitment

            
                        
                    
                    
                    Candidates

                        
                           
                    
                    
                    Vacancies

                        
                           
                    
                                                
                                         
            
            
        
            My Info

            
                                    
                        
                            
                                         
            
            
        
            Performance

            
                        
                    
                    
                    Configure

                        
                            

                                
                                    KPIs

                                
                                    Trackers

                                
                             

                        
                           
                    
                    
                    Manage Reviews

                        
                            

                                
                                    Manage Reviews

                                
                                    My Reviews

                                
                                    Review List

                                
                             

                        
                           
                    
                    
                    My Trackers

                        
                           
                    
                    
                    Employee Trackers

                        
                           
                    
                                                
                                         
            
            
        
            Dashboard

            
                                    
                        
                            
                                         
            
            
        
            Directory

            
                                    
                        
                            
                                         
            
            
        
            Maintenance

            
                        
                    
                    
                    Purge Records

                        
                            

                                
                                    Employee Records

                                
                                    Candidate Records

                                
                             

                        
                           
                    
                    
                    Access Records

                        
                           
                    
                                                
                                         
            
            
        
            Buzz

            
                                    
                        
                            
                                         
            
            
                    
     
    
 
            

                  

    .loadmask {
        top:0;
        left:0;
        -moz-opacity: 0.5;
        opacity: .50;
        filter: alpha(opacity=50);
        background-color: #CCC;
        width: 100%;
        height: 100%;
        zoom: 1;
        background: #fbfbfb url(&quot;/webres_62d518d1b0eb91.16069435/orangehrmDashboardPlugin/images/loading.gif&quot;) no-repeat center;
    }


    
        Dashboard
    
    
                            
                
                                        
                                                                                
                                
                                    Quick Launch
                                    

    .quickLinkText{
        display: block;
        text-align: center;
        color: black;
        font-weight:bold;
    }
    a:hover, a:visited, a:link, a:active{
        text-decoration: none;
    }
    div.quickLaunge{
        width: 100px;
        height: 80px;
        vertical-align:middle; 
        text-align:center
    }
    div.quickLaunge img{
        width: 50px;
        height: 50px;
    }
    table.quickLaungeContainer{
        width: auto;
    }



    
        
                                
                                                    
                                
                                    
                                        
                                        Assign Leave
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        Leave List
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        Timesheets
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        Apply Leave
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        My Leave
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        My Timesheet
                                    
                                                        
                                                
                                        
                
        
    



    $(document).ready(function () {
        // hover color change effect
        $(&quot;#dashboard-quick-launch-panel-slider li&quot;).hover(function () {
            $(this).animate({opacity: 0.90}, 100, function () {
                $(this).animate({opacity: 1}, 0);
            });
        });
        // Trigger mouse move event over the 'menu_holder'.
        $(&quot;#dashboard-quick-launch-panel-menu_holder&quot;).mousemove(function (e) {
            // Enable scroll function only when the height of the 'slider' or menu is greater than the 'menu_holder'.
            if ($(this).height() &lt; $(&quot;#dashboard-quick-launch-panel-slider&quot;).height()) {
                // Calculate the distance value from the 'menu_holder' y pos and page Y pos.
                var distance = e.pageY - $(this).offset().top;
                // Get the percentage value with respect to the Mouse Y on the 'menu_holder'.
                var percentage = distance / $(this).height();
                // Calculate the new Y position of the 'slider'. 
                var targetY = -Math.round(($(&quot;#dashboard-quick-launch-panel-slider&quot;).height() - $(this).height()) * percentage);
                // With jQuery easing funtion from easing plugin.
                $('#dashboard-quick-launch-panel-slider').animate({top: [targetY + &quot;px&quot;, &quot;easeOutCirc&quot;]}, {queue: false, duration: 200});
                // Without easeing function. by default jQuery have 'swing'.
                //$('#slider').animate({top: [targetY+&quot;px&quot;, &quot;easeOutCirc&quot;]}, { queue:false, duration:200 });
            }
        });
    });

                                 
                            

                                            
                
            
                        
                
                                        
                                                                                
                                
                                    Employee Distribution by Subunit
                                    
    






    69%10%
       
        
        
        

    
        $(document).ready(function () {

            var data = {&quot;Not assigned to Subunits&quot;:66,&quot;Administration&quot;:2,&quot;Client Services&quot;:3,&quot;Engineering&quot;:10,&quot;Finance&quot;:4,&quot;Human Resources&quot;:5,&quot;Sales &amp; Marketing&quot;:6};
            var graph = {&quot;legend&quot;:{&quot;labels&quot;:[&quot;Not assigned to Subunits&quot;,&quot;Administration&quot;,&quot;Client Services&quot;,&quot;Engineering&quot;,&quot;Finance&quot;,&quot;Human Resources&quot;,&quot;Sales &amp;amp; Marketing&quot;],&quot;noOfColumns&quot;:7,&quot;legendDivId&quot;:&quot;div_legend_pim_employee_distribution&quot;,&quot;useSeparateContainer&quot;:true}};


            var properties = {&quot;show-legend&quot;:true,&quot;show-labels&quot;:true,&quot;interactive&quot;:true,&quot;suffixForValueHover&quot;:&quot;Employee(s)&quot;};

            visualizePieChart(data, graph, properties, 'div_graph_display_emp_distribution');        });
       

    

        $(document).ready(function () {
            var moduleUrl = '/index.php/dashboard/employeeDistribution';
            var divId = 'dashboard__employeeDistribution';
            $.ajax({
                url: moduleUrl,
                success: function (obj) {
                    $(&quot;#&quot; + divId).html(obj);
                },
                complete: function () {
                    $(&quot;#&quot; + divId).removeClass('loadmask');
                }
            });
        });
    
                                 
                            

                                                                                
                                
                                    Legend
                                    
    .task-list-group-panel-menu_holder{
    margin:0;
    padding: 5px;
    padding-top: 8px;
    /*background:white;*/
    overflow-y:auto; /* make the overflow as hidden, like a mask layer*/
    overflow-x: hidden;
    height:100%; /* Height of the menu holder, same as the container*/
    position:relative;
    /*width:100%;*/
}


    Not assigned to SubunitsAdministrationClient ServicesEngineeringFinanceHuman ResourcesSales &amp; Marketing
                                 
                            

                                                                                
                                
                                    Pending Leave Requests
                                    
    
    
        
            
                                        No Records are Available
                              
        
    
    
        
            
                 
                     3 month(s)                
                 
                    Total : 0 / 0                                
            
        
    



    $(document).ready(function() {
        // hover color change effect
        $(&quot;#task-list-group-panel-slider li&quot;).hover(function() {
            $(this).animate({opacity: 0.90}, 100, function(){ 
                $(this).animate({opacity: 1}, 0);
            } );
        });     
    });


    

        $(document).ready(function () {
            var moduleUrl = '/index.php/dashboard/pendingLeaveRequests';
            var divId = 'dashboard__pendingLeaveRequests';
            $.ajax({
                url: moduleUrl,
                success: function (obj) {
                    $(&quot;#&quot; + divId).html(obj);
                },
                complete: function () {
                    $(&quot;#&quot; + divId).removeClass('loadmask');
                }
            });
        });
    
                                 
                            

                                            
                
            
                


             

        </value>
      <webElementGuid>90336139-00b1-4c7e-a473-d69f9949a4cf</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;wrapper&quot;)</value>
      <webElementGuid>d62e3f3b-d8f7-4501-9fad-e6b30a2e94e4</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='wrapper']</value>
      <webElementGuid>50bd6a22-7282-49f0-bc49-19eec190af89</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div</value>
      <webElementGuid>7d3a21d5-eed2-42aa-981c-e8099aeaef8d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'wrapper' and (text() = concat(&quot;

            
                
                Welcome ramya
                


    
        
        
    




    

        
        
            
                
                                    
                No new notifications
                
                                    
            
        
    

    
        
            
                
                                    
            

        

        
            


        

        
            

        
    




    
        
            

        
            
        

    



    
        
            Success!        
        
            Successfully Saved        
        
            Successfully Shared        
        
            Successfully Deleted        

    




    
        
            
        
    





    
        People who shared this post    
    
        
            
        

    





    
        People who like this post    
    
        
            
        

    




    var getAccessUrl = &quot; , &quot;'&quot; , &quot;/index.php/buzz/getLogedToBuzz&quot; , &quot;'&quot; , &quot;;
    var loginpageURL = &quot; , &quot;'&quot; , &quot;/index.php/auth/login&quot; , &quot;'&quot; , &quot;;

    // buzzCommon.js
    var viewLikedEmployees = &quot; , &quot;'&quot; , &quot;/index.php/buzz/viewLikedEmployees&quot; , &quot;'&quot; , &quot;;
    var addBuzzCommentURL = &quot; , &quot;'&quot; , &quot;/index.php/buzz/addNewComment&quot; , &quot;'&quot; , &quot;;
    var shareShareURL = &quot; , &quot;'&quot; , &quot;/index.php/buzz/shareAPost&quot; , &quot;'&quot; , &quot;;
    var getLikedEmployeeListURL = &quot; , &quot;'&quot; , &quot;/index.php/buzz/getLikedEmployeeList&quot; , &quot;'&quot; , &quot;;

    // buzzNew.js
    var shareLikeURL = &quot; , &quot;'&quot; , &quot;/index.php/buzz/likeOnShare&quot; , &quot;'&quot; , &quot;;
    var commentLikeURL = &quot; , &quot;'&quot; , &quot;/index.php/buzz/likeOnComment&quot; , &quot;'&quot; , &quot;;
    var getSharedEmployeeListURL = &quot; , &quot;'&quot; , &quot;/index.php/buzz/getSharedEmployeeList&quot; , &quot;'&quot; , &quot;;

    // viewNotificationComponent.js
    var viewMoreShare = &quot; , &quot;'&quot; , &quot;/index.php/buzz/viewShare&quot; , &quot;'&quot; , &quot;;
    var buzzURL = &quot;/index.php/buzz/viewBuzz&quot;;
    var ClearNotificationURL = &quot;/index.php/buzz/clearNotificationAjax&quot;;
    var ClickOnNotificationIconURL = &quot;/index.php/buzz/clickOnNotificationIconAjax&quot;;
    var lang_NoNewNotifications = &quot; , &quot;'&quot; , &quot;No new notifications&quot; , &quot;'&quot; , &quot;;
    var lang_NotificationClearFailed = &quot; , &quot;'&quot; , &quot;Failed to clear notifications&quot; , &quot;'&quot; , &quot;;

                

    
        
    

                                    
                        
                    
                                
                    var marketplaceURL = &quot;/index.php/marketPlace/ohrmAddons&quot;;
                    var SubscriberURL = &quot;/index.php/pim/subscriber&quot;;
                
                
                    
                        
    About

    
        
            ×
            About
        
        
            
                
                    
                        
                            Company Name: OrangeHRM
                        
                        
                            Version: Orangehrm OS 4.10.1
                        
                        
                            Active Employees: 96
                        
                        
                            Employees Terminated: 0
                        
                    
                
                
                        
    



    jQuery(document).ready(function() {
        
               
        jQuery(&quot; , &quot;'&quot; , &quot;#aboutDisplayLink&quot; , &quot;'&quot; , &quot;).click(function(event) {
            event.stopImmediatePropagation();
            jQuery(&quot; , &quot;'&quot; , &quot;#messageToDisplayAbout&quot; , &quot;'&quot; , &quot;).css(
                    &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
            jQuery(&quot; , &quot;'&quot; , &quot;#displayAbout&quot; , &quot;'&quot; , &quot;).modal();
            jQuery(&quot; , &quot;'&quot; , &quot;#help-menu.panelContainer&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;display:block&quot; , &quot;'&quot; , &quot;);
            
            var test = jQuery(&quot; , &quot;'&quot; , &quot;.panelContainer&quot; , &quot;'&quot; , &quot;);
            jQuery(&quot; , &quot;'&quot; , &quot;#help-menu.panelContainer&quot; , &quot;'&quot; , &quot;).css(
                    &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
             
        });

        jQuery(&quot; , &quot;'&quot; , &quot;#heartbeatSubmitBtn&quot; , &quot;'&quot; , &quot;).click(function(event) {
            event.stopImmediatePropagation();
            jQuery(this).closest(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).ajaxSubmit(function() {
                jQuery(&quot; , &quot;'&quot; , &quot;#messageToDisplayAbout&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Saved&quot; , &quot;'&quot; , &quot;);

                if (jQuery(&quot; , &quot;'&quot; , &quot;#register_registration&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
                    jQuery(&quot; , &quot;'&quot; , &quot;#registration-section&quot; , &quot;'&quot; , &quot;).css(
                            &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
                }
                jQuery(&quot; , &quot;'&quot; , &quot;#displayAbout&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
                jQuery(&quot; , &quot;'&quot; , &quot;#messageToDisplayAbout&quot; , &quot;'&quot; , &quot;).css(
                        &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
                jQuery(&quot; , &quot;'&quot; , &quot;#welcome-menu&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
            });
        });

        jQuery(&quot; , &quot;'&quot; , &quot;#displayAbout&quot; , &quot;'&quot; , &quot;).click(function(event) {
            event.stopImmediatePropagation();
        });
        
        jQuery(&quot; , &quot;'&quot; , &quot;#heartbeatCancelBtn&quot; , &quot;'&quot; , &quot;).click(function(event) {
            event.stopImmediatePropagation();
             jQuery(&quot; , &quot;'&quot; , &quot;#welcome-menu&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
                 jQuery(&quot; , &quot;'&quot; , &quot;#displayAbout&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
        });

    })



                        Support
                        Logout
                    
                
                                


    svg path,
    svg rect{
        fill: #FF6700;
    }
    .svgcl{
        position: relative;
        left: -35px;
        top: -31px;
    }
    


    var inputDatePattern = &quot; , &quot;'&quot; , &quot;Y-m-d&quot; , &quot;'&quot; , &quot; ;
    var separatorString = &quot; , &quot;'&quot; , &quot;to&quot; , &quot;'&quot; , &quot;;
    $( document ).ready(function() {

        $(&quot;#loader-1&quot;).hide();
        empId = location.href[location.href.length-1];
        dates = $(&quot; , &quot;'&quot; , &quot;#startDates&quot; , &quot;'&quot; , &quot;).find(&quot;:selected&quot;).text().split(&quot; &quot;+separatorString+&quot; &quot;);
        startDate_timesheet = dates[0]+&quot; 00:00:00&quot;;
        endDate_timesheet   = dates[1]+&quot; 00:00:00&quot;;

        clientId  =     &quot;&quot;;
        clientSecret  = &quot;&quot;;
        clientUrl     = &quot;&quot;;
        successUrl  = &quot;&quot;;
        ajaxURL = &quot;/index.php/buzz/viewBuzz&quot;;
        var timeSheetStatus = $(&quot; , &quot;'&quot; , &quot;#timesheet_status&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;h2&quot; , &quot;'&quot; , &quot;).text();
        if(timeSheetStatus == &quot; , &quot;'&quot; , &quot;Status: Approved&quot; , &quot;'&quot; , &quot;){

            $(&quot; , &quot;'&quot; , &quot;.syncToggl&quot; , &quot;'&quot; , &quot;).hide();
        } else {
            $(&quot; , &quot;'&quot; , &quot;.syncToggl&quot; , &quot;'&quot; , &quot;).show();
        }

    });

        
    function ajaxSyc() {
        $(&quot;#loader-1&quot;).show();

        $.ajax({
                type: &quot;POST&quot;,
                url: ajaxURL,
                data: {
                    &quot; , &quot;'&quot; , &quot;employee_Id&quot; , &quot;'&quot; , &quot;:employeeId,
                    &quot; , &quot;'&quot; , &quot;startTime&quot; , &quot;'&quot; , &quot;: startDate_timesheet,
                    &quot; , &quot;'&quot; , &quot;endTime&quot; , &quot;'&quot; , &quot;: endDate_timesheet,
                    &quot; , &quot;'&quot; , &quot;timeFormat&quot; , &quot;'&quot; , &quot;: inputDatePattern,
                    &quot; , &quot;'&quot; , &quot;timeZone&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;GMT&quot; , &quot;'&quot; , &quot;+formatTimeZone()
                },
                contentType: &quot;application/x-www-form-urlencoded&quot;,

                success: function (msg, status, jqXHR) {

                    $(&quot;#loader-1&quot;).hide();
                    msg = JSON.parse(msg);
                    msgCode = msg.statusCode;
                    if (msgCode != null) {
                        if (msgCode == 101) {
                            displayMessages(&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,msg.description );
                        } else if (msgCode == 102) {

                            displayMessages(&quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;, msg.description);
                            setTimeout(function () {
                                location.reload();
                            }, 2000);

                        }
                    } else {
                        showErrorMsg();
                    }

                },
                error: function (XMLHttpRequest, textStatus, errorThrown) {
                    $(&quot;#loader-1&quot;).hide();
                    console.log(errorThrown);
                    showErrorMsg();
                }
            });
    }
    
    function startSyc() {
        $(&quot;#loader-1&quot;).show();

    $.ajax({

        type: &quot;POST&quot;,
        url: clientUrl,


        data: {
            &quot; , &quot;'&quot; , &quot;grant_type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;client_credentials&quot; , &quot;'&quot; , &quot;,
            &quot; , &quot;'&quot; , &quot;client_id&quot; , &quot;'&quot; , &quot;: clientId,
            &quot; , &quot;'&quot; , &quot;client_secret&quot; , &quot;'&quot; , &quot;: clientSecret
        },
        contentType: &quot;application/x-www-form-urlencoded&quot;,


        success: function (msg, status, jqXHR) {

            try {

                msg = $.parseJSON(jqXHR.responseText);

            } catch (err) {
                console.log(err);
                showErrorMsg();
            }

            $.ajax({
                type: &quot;POST&quot;,
                url: successUrl,
                beforeSend: function (xhr) {

                    xhr.setRequestHeader(&quot;Authorization&quot;, &quot;Bearer &quot; + msg.access_token);
                },

                data: {

                    &quot; , &quot;'&quot; , &quot;employee_Id&quot; , &quot;'&quot; , &quot;:employeeId,
                    &quot; , &quot;'&quot; , &quot;startTime&quot; , &quot;'&quot; , &quot;: startDate_timesheet,
                    &quot; , &quot;'&quot; , &quot;endTime&quot; , &quot;'&quot; , &quot;: endDate_timesheet,
                    &quot; , &quot;'&quot; , &quot;timeFormat&quot; , &quot;'&quot; , &quot;: inputDatePattern,
                    &quot; , &quot;'&quot; , &quot;timeZone&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;GMT&quot; , &quot;'&quot; , &quot;+formatTimeZone()
                },
                contentType: &quot;application/x-www-form-urlencoded&quot;,

                success: function (msg, status, jqXHR) {

                    $(&quot;#loader-1&quot;).hide();
                    msgCode = msg.statusCode;
                    if (msgCode != null) {
                        if (msgCode == 101) {
                            displayMessages(&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,msg.description );
                        } else if (msgCode == 102) {

                            displayMessages(&quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;, msg.description);
                            setTimeout(function () {
                                location.reload();
                            }, 2000);

                        }
                    } else {
                        showErrorMsg();
                    }

                },
                error: function (XMLHttpRequest, textStatus, errorThrown) {
                    $(&quot;#loader-1&quot;).hide();
                    console.log(errorThrown);
                    showErrorMsg();
                }
            });

        },
        error: function (XMLHttpRequest, textStatus, errorThrown) {
            $(&quot;#loader-1&quot;).hide();
            console.log(errorThrown);
            showErrorMsg();
        }


    });

    }

    function showErrorMsg(){
        displayMessages(&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Unable To Sync With Toggl&quot; , &quot;'&quot; , &quot; );
        setTimeout(function () {
            $(&quot; , &quot;'&quot; , &quot;#msgDiv&quot; , &quot;'&quot; , &quot;).remove();
        }, 3000);

    }




    
    
        
            
  
      
  
  
        
    

    
        
            ×
            Confirm Toggl Sync
        
        
            Any existing timesheet entry will be overwritten if record for same date is matched. Click ok to continue.
        
        
                            
                        
        
    






             

            

    
    
        
        
            Admin

            
                        
                    
                    
                    User Management

                        
                            

                                
                                    Users

                                
                             

                        
                           
                    
                    
                    Job

                        
                            

                                
                                    Job Titles

                                
                                    Pay Grades

                                
                                    Employment Status

                                
                                    Job Categories

                                
                                    Work Shifts

                                
                             

                        
                           
                    
                    
                    Organization

                        
                            

                                
                                    General Information

                                
                                    Locations

                                
                                    Structure

                                
                             

                        
                           
                    
                    
                    Qualifications

                        
                            

                                
                                    Skills

                                
                                    Education

                                
                                    Licenses

                                
                                    Languages

                                
                                    Memberships

                                
                             

                        
                           
                    
                    
                    Nationalities

                        
                           
                    
                    
                    Corporate Branding

                        
                           
                    
                    
                    Configuration

                        
                            

                                
                                    Email Configuration

                                
                                    Email Subscriptions

                                
                                    Localization

                                
                                    Language Packages

                                
                                    Modules

                                
                                    Social Media Authentication

                                
                                    Register OAuth Client

                                
                             

                        
                           
                    
                                                
                                         
            
            
        
            PIM

            
                        
                    
                    
                    Configuration

                        
                            

                                
                                    Optional Fields

                                
                                    Custom Fields

                                
                                    Data Import

                                
                                    Reporting Methods

                                
                                    Termination Reasons

                                
                             

                        
                           
                    
                    
                    Employee List

                        
                           
                    
                    
                    Add Employee

                        
                           
                    
                    
                    Reports

                        
                           
                    
                                                
                                         
            
            
        
            Leave

            
                        
                    
                    
                    Apply

                        
                           
                    
                    
                    My Leave

                        
                           
                    
                    
                    Entitlements

                        
                            

                                
                                    Add Entitlements

                                
                                    Employee Entitlements

                                
                                    My Entitlements

                                
                             

                        
                           
                    
                    
                    Reports

                        
                            

                                
                                    Leave Entitlements and Usage Report

                                
                                    My Leave Entitlements and Usage Report

                                
                             

                        
                           
                    
                    
                    Configure

                        
                            

                                
                                    Leave Period

                                
                                    Leave Types

                                
                                    Work Week

                                
                                    Holidays

                                
                             

                        
                           
                    
                    
                    Leave List

                        
                           
                    
                    
                    Assign Leave

                        
                           
                    
                                                
                                         
            
            
        
            Time

            
                        
                    
                    
                    Timesheets

                        
                            

                                
                                    My Timesheets

                                
                                    Employee Timesheets

                                
                             

                        
                           
                    
                    
                    Attendance

                        
                            

                                
                                    My Records

                                
                                    Punch In/Out

                                
                                    Employee Records

                                
                                    Configuration

                                
                             

                        
                           
                    
                    
                    Reports

                        
                            

                                
                                    Project Reports

                                
                                    Employee Reports

                                
                                    Attendance Summary

                                
                             

                        
                           
                    
                    
                    Project Info

                        
                            

                                
                                    Customers

                                
                                    Projects

                                
                             

                        
                           
                    
                                                
                                         
            
            
        
            Recruitment

            
                        
                    
                    
                    Candidates

                        
                           
                    
                    
                    Vacancies

                        
                           
                    
                                                
                                         
            
            
        
            My Info

            
                                    
                        
                            
                                         
            
            
        
            Performance

            
                        
                    
                    
                    Configure

                        
                            

                                
                                    KPIs

                                
                                    Trackers

                                
                             

                        
                           
                    
                    
                    Manage Reviews

                        
                            

                                
                                    Manage Reviews

                                
                                    My Reviews

                                
                                    Review List

                                
                             

                        
                           
                    
                    
                    My Trackers

                        
                           
                    
                    
                    Employee Trackers

                        
                           
                    
                                                
                                         
            
            
        
            Dashboard

            
                                    
                        
                            
                                         
            
            
        
            Directory

            
                                    
                        
                            
                                         
            
            
        
            Maintenance

            
                        
                    
                    
                    Purge Records

                        
                            

                                
                                    Employee Records

                                
                                    Candidate Records

                                
                             

                        
                           
                    
                    
                    Access Records

                        
                           
                    
                                                
                                         
            
            
        
            Buzz

            
                                    
                        
                            
                                         
            
            
                    
     
    
 
            

                  

    .loadmask {
        top:0;
        left:0;
        -moz-opacity: 0.5;
        opacity: .50;
        filter: alpha(opacity=50);
        background-color: #CCC;
        width: 100%;
        height: 100%;
        zoom: 1;
        background: #fbfbfb url(&quot;/webres_62d518d1b0eb91.16069435/orangehrmDashboardPlugin/images/loading.gif&quot;) no-repeat center;
    }


    
        Dashboard
    
    
                            
                
                                        
                                                                                
                                
                                    Quick Launch
                                    

    .quickLinkText{
        display: block;
        text-align: center;
        color: black;
        font-weight:bold;
    }
    a:hover, a:visited, a:link, a:active{
        text-decoration: none;
    }
    div.quickLaunge{
        width: 100px;
        height: 80px;
        vertical-align:middle; 
        text-align:center
    }
    div.quickLaunge img{
        width: 50px;
        height: 50px;
    }
    table.quickLaungeContainer{
        width: auto;
    }



    
        
                                
                                                    
                                
                                    
                                        
                                        Assign Leave
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        Leave List
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        Timesheets
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        Apply Leave
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        My Leave
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        My Timesheet
                                    
                                                        
                                                
                                        
                
        
    



    $(document).ready(function () {
        // hover color change effect
        $(&quot;#dashboard-quick-launch-panel-slider li&quot;).hover(function () {
            $(this).animate({opacity: 0.90}, 100, function () {
                $(this).animate({opacity: 1}, 0);
            });
        });
        // Trigger mouse move event over the &quot; , &quot;'&quot; , &quot;menu_holder&quot; , &quot;'&quot; , &quot;.
        $(&quot;#dashboard-quick-launch-panel-menu_holder&quot;).mousemove(function (e) {
            // Enable scroll function only when the height of the &quot; , &quot;'&quot; , &quot;slider&quot; , &quot;'&quot; , &quot; or menu is greater than the &quot; , &quot;'&quot; , &quot;menu_holder&quot; , &quot;'&quot; , &quot;.
            if ($(this).height() &lt; $(&quot;#dashboard-quick-launch-panel-slider&quot;).height()) {
                // Calculate the distance value from the &quot; , &quot;'&quot; , &quot;menu_holder&quot; , &quot;'&quot; , &quot; y pos and page Y pos.
                var distance = e.pageY - $(this).offset().top;
                // Get the percentage value with respect to the Mouse Y on the &quot; , &quot;'&quot; , &quot;menu_holder&quot; , &quot;'&quot; , &quot;.
                var percentage = distance / $(this).height();
                // Calculate the new Y position of the &quot; , &quot;'&quot; , &quot;slider&quot; , &quot;'&quot; , &quot;. 
                var targetY = -Math.round(($(&quot;#dashboard-quick-launch-panel-slider&quot;).height() - $(this).height()) * percentage);
                // With jQuery easing funtion from easing plugin.
                $(&quot; , &quot;'&quot; , &quot;#dashboard-quick-launch-panel-slider&quot; , &quot;'&quot; , &quot;).animate({top: [targetY + &quot;px&quot;, &quot;easeOutCirc&quot;]}, {queue: false, duration: 200});
                // Without easeing function. by default jQuery have &quot; , &quot;'&quot; , &quot;swing&quot; , &quot;'&quot; , &quot;.
                //$(&quot; , &quot;'&quot; , &quot;#slider&quot; , &quot;'&quot; , &quot;).animate({top: [targetY+&quot;px&quot;, &quot;easeOutCirc&quot;]}, { queue:false, duration:200 });
            }
        });
    });

                                 
                            

                                            
                
            
                        
                
                                        
                                                                                
                                
                                    Employee Distribution by Subunit
                                    
    






    69%10%
       
        
        
        

    
        $(document).ready(function () {

            var data = {&quot;Not assigned to Subunits&quot;:66,&quot;Administration&quot;:2,&quot;Client Services&quot;:3,&quot;Engineering&quot;:10,&quot;Finance&quot;:4,&quot;Human Resources&quot;:5,&quot;Sales &amp; Marketing&quot;:6};
            var graph = {&quot;legend&quot;:{&quot;labels&quot;:[&quot;Not assigned to Subunits&quot;,&quot;Administration&quot;,&quot;Client Services&quot;,&quot;Engineering&quot;,&quot;Finance&quot;,&quot;Human Resources&quot;,&quot;Sales &amp;amp; Marketing&quot;],&quot;noOfColumns&quot;:7,&quot;legendDivId&quot;:&quot;div_legend_pim_employee_distribution&quot;,&quot;useSeparateContainer&quot;:true}};


            var properties = {&quot;show-legend&quot;:true,&quot;show-labels&quot;:true,&quot;interactive&quot;:true,&quot;suffixForValueHover&quot;:&quot;Employee(s)&quot;};

            visualizePieChart(data, graph, properties, &quot; , &quot;'&quot; , &quot;div_graph_display_emp_distribution&quot; , &quot;'&quot; , &quot;);        });
       

    

        $(document).ready(function () {
            var moduleUrl = &quot; , &quot;'&quot; , &quot;/index.php/dashboard/employeeDistribution&quot; , &quot;'&quot; , &quot;;
            var divId = &quot; , &quot;'&quot; , &quot;dashboard__employeeDistribution&quot; , &quot;'&quot; , &quot;;
            $.ajax({
                url: moduleUrl,
                success: function (obj) {
                    $(&quot;#&quot; + divId).html(obj);
                },
                complete: function () {
                    $(&quot;#&quot; + divId).removeClass(&quot; , &quot;'&quot; , &quot;loadmask&quot; , &quot;'&quot; , &quot;);
                }
            });
        });
    
                                 
                            

                                                                                
                                
                                    Legend
                                    
    .task-list-group-panel-menu_holder{
    margin:0;
    padding: 5px;
    padding-top: 8px;
    /*background:white;*/
    overflow-y:auto; /* make the overflow as hidden, like a mask layer*/
    overflow-x: hidden;
    height:100%; /* Height of the menu holder, same as the container*/
    position:relative;
    /*width:100%;*/
}


    Not assigned to SubunitsAdministrationClient ServicesEngineeringFinanceHuman ResourcesSales &amp; Marketing
                                 
                            

                                                                                
                                
                                    Pending Leave Requests
                                    
    
    
        
            
                                        No Records are Available
                              
        
    
    
        
            
                 
                     3 month(s)                
                 
                    Total : 0 / 0                                
            
        
    



    $(document).ready(function() {
        // hover color change effect
        $(&quot;#task-list-group-panel-slider li&quot;).hover(function() {
            $(this).animate({opacity: 0.90}, 100, function(){ 
                $(this).animate({opacity: 1}, 0);
            } );
        });     
    });


    

        $(document).ready(function () {
            var moduleUrl = &quot; , &quot;'&quot; , &quot;/index.php/dashboard/pendingLeaveRequests&quot; , &quot;'&quot; , &quot;;
            var divId = &quot; , &quot;'&quot; , &quot;dashboard__pendingLeaveRequests&quot; , &quot;'&quot; , &quot;;
            $.ajax({
                url: moduleUrl,
                success: function (obj) {
                    $(&quot;#&quot; + divId).html(obj);
                },
                complete: function () {
                    $(&quot;#&quot; + divId).removeClass(&quot; , &quot;'&quot; , &quot;loadmask&quot; , &quot;'&quot; , &quot;);
                }
            });
        });
    
                                 
                            

                                            
                
            
                


             

        &quot;) or . = concat(&quot;

            
                
                Welcome ramya
                


    
        
        
    




    

        
        
            
                
                                    
                No new notifications
                
                                    
            
        
    

    
        
            
                
                                    
            

        

        
            


        

        
            

        
    




    
        
            

        
            
        

    



    
        
            Success!        
        
            Successfully Saved        
        
            Successfully Shared        
        
            Successfully Deleted        

    




    
        
            
        
    





    
        People who shared this post    
    
        
            
        

    





    
        People who like this post    
    
        
            
        

    




    var getAccessUrl = &quot; , &quot;'&quot; , &quot;/index.php/buzz/getLogedToBuzz&quot; , &quot;'&quot; , &quot;;
    var loginpageURL = &quot; , &quot;'&quot; , &quot;/index.php/auth/login&quot; , &quot;'&quot; , &quot;;

    // buzzCommon.js
    var viewLikedEmployees = &quot; , &quot;'&quot; , &quot;/index.php/buzz/viewLikedEmployees&quot; , &quot;'&quot; , &quot;;
    var addBuzzCommentURL = &quot; , &quot;'&quot; , &quot;/index.php/buzz/addNewComment&quot; , &quot;'&quot; , &quot;;
    var shareShareURL = &quot; , &quot;'&quot; , &quot;/index.php/buzz/shareAPost&quot; , &quot;'&quot; , &quot;;
    var getLikedEmployeeListURL = &quot; , &quot;'&quot; , &quot;/index.php/buzz/getLikedEmployeeList&quot; , &quot;'&quot; , &quot;;

    // buzzNew.js
    var shareLikeURL = &quot; , &quot;'&quot; , &quot;/index.php/buzz/likeOnShare&quot; , &quot;'&quot; , &quot;;
    var commentLikeURL = &quot; , &quot;'&quot; , &quot;/index.php/buzz/likeOnComment&quot; , &quot;'&quot; , &quot;;
    var getSharedEmployeeListURL = &quot; , &quot;'&quot; , &quot;/index.php/buzz/getSharedEmployeeList&quot; , &quot;'&quot; , &quot;;

    // viewNotificationComponent.js
    var viewMoreShare = &quot; , &quot;'&quot; , &quot;/index.php/buzz/viewShare&quot; , &quot;'&quot; , &quot;;
    var buzzURL = &quot;/index.php/buzz/viewBuzz&quot;;
    var ClearNotificationURL = &quot;/index.php/buzz/clearNotificationAjax&quot;;
    var ClickOnNotificationIconURL = &quot;/index.php/buzz/clickOnNotificationIconAjax&quot;;
    var lang_NoNewNotifications = &quot; , &quot;'&quot; , &quot;No new notifications&quot; , &quot;'&quot; , &quot;;
    var lang_NotificationClearFailed = &quot; , &quot;'&quot; , &quot;Failed to clear notifications&quot; , &quot;'&quot; , &quot;;

                

    
        
    

                                    
                        
                    
                                
                    var marketplaceURL = &quot;/index.php/marketPlace/ohrmAddons&quot;;
                    var SubscriberURL = &quot;/index.php/pim/subscriber&quot;;
                
                
                    
                        
    About

    
        
            ×
            About
        
        
            
                
                    
                        
                            Company Name: OrangeHRM
                        
                        
                            Version: Orangehrm OS 4.10.1
                        
                        
                            Active Employees: 96
                        
                        
                            Employees Terminated: 0
                        
                    
                
                
                        
    



    jQuery(document).ready(function() {
        
               
        jQuery(&quot; , &quot;'&quot; , &quot;#aboutDisplayLink&quot; , &quot;'&quot; , &quot;).click(function(event) {
            event.stopImmediatePropagation();
            jQuery(&quot; , &quot;'&quot; , &quot;#messageToDisplayAbout&quot; , &quot;'&quot; , &quot;).css(
                    &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
            jQuery(&quot; , &quot;'&quot; , &quot;#displayAbout&quot; , &quot;'&quot; , &quot;).modal();
            jQuery(&quot; , &quot;'&quot; , &quot;#help-menu.panelContainer&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;display:block&quot; , &quot;'&quot; , &quot;);
            
            var test = jQuery(&quot; , &quot;'&quot; , &quot;.panelContainer&quot; , &quot;'&quot; , &quot;);
            jQuery(&quot; , &quot;'&quot; , &quot;#help-menu.panelContainer&quot; , &quot;'&quot; , &quot;).css(
                    &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
             
        });

        jQuery(&quot; , &quot;'&quot; , &quot;#heartbeatSubmitBtn&quot; , &quot;'&quot; , &quot;).click(function(event) {
            event.stopImmediatePropagation();
            jQuery(this).closest(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).ajaxSubmit(function() {
                jQuery(&quot; , &quot;'&quot; , &quot;#messageToDisplayAbout&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Saved&quot; , &quot;'&quot; , &quot;);

                if (jQuery(&quot; , &quot;'&quot; , &quot;#register_registration&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
                    jQuery(&quot; , &quot;'&quot; , &quot;#registration-section&quot; , &quot;'&quot; , &quot;).css(
                            &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
                }
                jQuery(&quot; , &quot;'&quot; , &quot;#displayAbout&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
                jQuery(&quot; , &quot;'&quot; , &quot;#messageToDisplayAbout&quot; , &quot;'&quot; , &quot;).css(
                        &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
                jQuery(&quot; , &quot;'&quot; , &quot;#welcome-menu&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
            });
        });

        jQuery(&quot; , &quot;'&quot; , &quot;#displayAbout&quot; , &quot;'&quot; , &quot;).click(function(event) {
            event.stopImmediatePropagation();
        });
        
        jQuery(&quot; , &quot;'&quot; , &quot;#heartbeatCancelBtn&quot; , &quot;'&quot; , &quot;).click(function(event) {
            event.stopImmediatePropagation();
             jQuery(&quot; , &quot;'&quot; , &quot;#welcome-menu&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
                 jQuery(&quot; , &quot;'&quot; , &quot;#displayAbout&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
        });

    })



                        Support
                        Logout
                    
                
                                


    svg path,
    svg rect{
        fill: #FF6700;
    }
    .svgcl{
        position: relative;
        left: -35px;
        top: -31px;
    }
    


    var inputDatePattern = &quot; , &quot;'&quot; , &quot;Y-m-d&quot; , &quot;'&quot; , &quot; ;
    var separatorString = &quot; , &quot;'&quot; , &quot;to&quot; , &quot;'&quot; , &quot;;
    $( document ).ready(function() {

        $(&quot;#loader-1&quot;).hide();
        empId = location.href[location.href.length-1];
        dates = $(&quot; , &quot;'&quot; , &quot;#startDates&quot; , &quot;'&quot; , &quot;).find(&quot;:selected&quot;).text().split(&quot; &quot;+separatorString+&quot; &quot;);
        startDate_timesheet = dates[0]+&quot; 00:00:00&quot;;
        endDate_timesheet   = dates[1]+&quot; 00:00:00&quot;;

        clientId  =     &quot;&quot;;
        clientSecret  = &quot;&quot;;
        clientUrl     = &quot;&quot;;
        successUrl  = &quot;&quot;;
        ajaxURL = &quot;/index.php/buzz/viewBuzz&quot;;
        var timeSheetStatus = $(&quot; , &quot;'&quot; , &quot;#timesheet_status&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;h2&quot; , &quot;'&quot; , &quot;).text();
        if(timeSheetStatus == &quot; , &quot;'&quot; , &quot;Status: Approved&quot; , &quot;'&quot; , &quot;){

            $(&quot; , &quot;'&quot; , &quot;.syncToggl&quot; , &quot;'&quot; , &quot;).hide();
        } else {
            $(&quot; , &quot;'&quot; , &quot;.syncToggl&quot; , &quot;'&quot; , &quot;).show();
        }

    });

        
    function ajaxSyc() {
        $(&quot;#loader-1&quot;).show();

        $.ajax({
                type: &quot;POST&quot;,
                url: ajaxURL,
                data: {
                    &quot; , &quot;'&quot; , &quot;employee_Id&quot; , &quot;'&quot; , &quot;:employeeId,
                    &quot; , &quot;'&quot; , &quot;startTime&quot; , &quot;'&quot; , &quot;: startDate_timesheet,
                    &quot; , &quot;'&quot; , &quot;endTime&quot; , &quot;'&quot; , &quot;: endDate_timesheet,
                    &quot; , &quot;'&quot; , &quot;timeFormat&quot; , &quot;'&quot; , &quot;: inputDatePattern,
                    &quot; , &quot;'&quot; , &quot;timeZone&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;GMT&quot; , &quot;'&quot; , &quot;+formatTimeZone()
                },
                contentType: &quot;application/x-www-form-urlencoded&quot;,

                success: function (msg, status, jqXHR) {

                    $(&quot;#loader-1&quot;).hide();
                    msg = JSON.parse(msg);
                    msgCode = msg.statusCode;
                    if (msgCode != null) {
                        if (msgCode == 101) {
                            displayMessages(&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,msg.description );
                        } else if (msgCode == 102) {

                            displayMessages(&quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;, msg.description);
                            setTimeout(function () {
                                location.reload();
                            }, 2000);

                        }
                    } else {
                        showErrorMsg();
                    }

                },
                error: function (XMLHttpRequest, textStatus, errorThrown) {
                    $(&quot;#loader-1&quot;).hide();
                    console.log(errorThrown);
                    showErrorMsg();
                }
            });
    }
    
    function startSyc() {
        $(&quot;#loader-1&quot;).show();

    $.ajax({

        type: &quot;POST&quot;,
        url: clientUrl,


        data: {
            &quot; , &quot;'&quot; , &quot;grant_type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;client_credentials&quot; , &quot;'&quot; , &quot;,
            &quot; , &quot;'&quot; , &quot;client_id&quot; , &quot;'&quot; , &quot;: clientId,
            &quot; , &quot;'&quot; , &quot;client_secret&quot; , &quot;'&quot; , &quot;: clientSecret
        },
        contentType: &quot;application/x-www-form-urlencoded&quot;,


        success: function (msg, status, jqXHR) {

            try {

                msg = $.parseJSON(jqXHR.responseText);

            } catch (err) {
                console.log(err);
                showErrorMsg();
            }

            $.ajax({
                type: &quot;POST&quot;,
                url: successUrl,
                beforeSend: function (xhr) {

                    xhr.setRequestHeader(&quot;Authorization&quot;, &quot;Bearer &quot; + msg.access_token);
                },

                data: {

                    &quot; , &quot;'&quot; , &quot;employee_Id&quot; , &quot;'&quot; , &quot;:employeeId,
                    &quot; , &quot;'&quot; , &quot;startTime&quot; , &quot;'&quot; , &quot;: startDate_timesheet,
                    &quot; , &quot;'&quot; , &quot;endTime&quot; , &quot;'&quot; , &quot;: endDate_timesheet,
                    &quot; , &quot;'&quot; , &quot;timeFormat&quot; , &quot;'&quot; , &quot;: inputDatePattern,
                    &quot; , &quot;'&quot; , &quot;timeZone&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;GMT&quot; , &quot;'&quot; , &quot;+formatTimeZone()
                },
                contentType: &quot;application/x-www-form-urlencoded&quot;,

                success: function (msg, status, jqXHR) {

                    $(&quot;#loader-1&quot;).hide();
                    msgCode = msg.statusCode;
                    if (msgCode != null) {
                        if (msgCode == 101) {
                            displayMessages(&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,msg.description );
                        } else if (msgCode == 102) {

                            displayMessages(&quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;, msg.description);
                            setTimeout(function () {
                                location.reload();
                            }, 2000);

                        }
                    } else {
                        showErrorMsg();
                    }

                },
                error: function (XMLHttpRequest, textStatus, errorThrown) {
                    $(&quot;#loader-1&quot;).hide();
                    console.log(errorThrown);
                    showErrorMsg();
                }
            });

        },
        error: function (XMLHttpRequest, textStatus, errorThrown) {
            $(&quot;#loader-1&quot;).hide();
            console.log(errorThrown);
            showErrorMsg();
        }


    });

    }

    function showErrorMsg(){
        displayMessages(&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Unable To Sync With Toggl&quot; , &quot;'&quot; , &quot; );
        setTimeout(function () {
            $(&quot; , &quot;'&quot; , &quot;#msgDiv&quot; , &quot;'&quot; , &quot;).remove();
        }, 3000);

    }




    
    
        
            
  
      
  
  
        
    

    
        
            ×
            Confirm Toggl Sync
        
        
            Any existing timesheet entry will be overwritten if record for same date is matched. Click ok to continue.
        
        
                            
                        
        
    






             

            

    
    
        
        
            Admin

            
                        
                    
                    
                    User Management

                        
                            

                                
                                    Users

                                
                             

                        
                           
                    
                    
                    Job

                        
                            

                                
                                    Job Titles

                                
                                    Pay Grades

                                
                                    Employment Status

                                
                                    Job Categories

                                
                                    Work Shifts

                                
                             

                        
                           
                    
                    
                    Organization

                        
                            

                                
                                    General Information

                                
                                    Locations

                                
                                    Structure

                                
                             

                        
                           
                    
                    
                    Qualifications

                        
                            

                                
                                    Skills

                                
                                    Education

                                
                                    Licenses

                                
                                    Languages

                                
                                    Memberships

                                
                             

                        
                           
                    
                    
                    Nationalities

                        
                           
                    
                    
                    Corporate Branding

                        
                           
                    
                    
                    Configuration

                        
                            

                                
                                    Email Configuration

                                
                                    Email Subscriptions

                                
                                    Localization

                                
                                    Language Packages

                                
                                    Modules

                                
                                    Social Media Authentication

                                
                                    Register OAuth Client

                                
                             

                        
                           
                    
                                                
                                         
            
            
        
            PIM

            
                        
                    
                    
                    Configuration

                        
                            

                                
                                    Optional Fields

                                
                                    Custom Fields

                                
                                    Data Import

                                
                                    Reporting Methods

                                
                                    Termination Reasons

                                
                             

                        
                           
                    
                    
                    Employee List

                        
                           
                    
                    
                    Add Employee

                        
                           
                    
                    
                    Reports

                        
                           
                    
                                                
                                         
            
            
        
            Leave

            
                        
                    
                    
                    Apply

                        
                           
                    
                    
                    My Leave

                        
                           
                    
                    
                    Entitlements

                        
                            

                                
                                    Add Entitlements

                                
                                    Employee Entitlements

                                
                                    My Entitlements

                                
                             

                        
                           
                    
                    
                    Reports

                        
                            

                                
                                    Leave Entitlements and Usage Report

                                
                                    My Leave Entitlements and Usage Report

                                
                             

                        
                           
                    
                    
                    Configure

                        
                            

                                
                                    Leave Period

                                
                                    Leave Types

                                
                                    Work Week

                                
                                    Holidays

                                
                             

                        
                           
                    
                    
                    Leave List

                        
                           
                    
                    
                    Assign Leave

                        
                           
                    
                                                
                                         
            
            
        
            Time

            
                        
                    
                    
                    Timesheets

                        
                            

                                
                                    My Timesheets

                                
                                    Employee Timesheets

                                
                             

                        
                           
                    
                    
                    Attendance

                        
                            

                                
                                    My Records

                                
                                    Punch In/Out

                                
                                    Employee Records

                                
                                    Configuration

                                
                             

                        
                           
                    
                    
                    Reports

                        
                            

                                
                                    Project Reports

                                
                                    Employee Reports

                                
                                    Attendance Summary

                                
                             

                        
                           
                    
                    
                    Project Info

                        
                            

                                
                                    Customers

                                
                                    Projects

                                
                             

                        
                           
                    
                                                
                                         
            
            
        
            Recruitment

            
                        
                    
                    
                    Candidates

                        
                           
                    
                    
                    Vacancies

                        
                           
                    
                                                
                                         
            
            
        
            My Info

            
                                    
                        
                            
                                         
            
            
        
            Performance

            
                        
                    
                    
                    Configure

                        
                            

                                
                                    KPIs

                                
                                    Trackers

                                
                             

                        
                           
                    
                    
                    Manage Reviews

                        
                            

                                
                                    Manage Reviews

                                
                                    My Reviews

                                
                                    Review List

                                
                             

                        
                           
                    
                    
                    My Trackers

                        
                           
                    
                    
                    Employee Trackers

                        
                           
                    
                                                
                                         
            
            
        
            Dashboard

            
                                    
                        
                            
                                         
            
            
        
            Directory

            
                                    
                        
                            
                                         
            
            
        
            Maintenance

            
                        
                    
                    
                    Purge Records

                        
                            

                                
                                    Employee Records

                                
                                    Candidate Records

                                
                             

                        
                           
                    
                    
                    Access Records

                        
                           
                    
                                                
                                         
            
            
        
            Buzz

            
                                    
                        
                            
                                         
            
            
                    
     
    
 
            

                  

    .loadmask {
        top:0;
        left:0;
        -moz-opacity: 0.5;
        opacity: .50;
        filter: alpha(opacity=50);
        background-color: #CCC;
        width: 100%;
        height: 100%;
        zoom: 1;
        background: #fbfbfb url(&quot;/webres_62d518d1b0eb91.16069435/orangehrmDashboardPlugin/images/loading.gif&quot;) no-repeat center;
    }


    
        Dashboard
    
    
                            
                
                                        
                                                                                
                                
                                    Quick Launch
                                    

    .quickLinkText{
        display: block;
        text-align: center;
        color: black;
        font-weight:bold;
    }
    a:hover, a:visited, a:link, a:active{
        text-decoration: none;
    }
    div.quickLaunge{
        width: 100px;
        height: 80px;
        vertical-align:middle; 
        text-align:center
    }
    div.quickLaunge img{
        width: 50px;
        height: 50px;
    }
    table.quickLaungeContainer{
        width: auto;
    }



    
        
                                
                                                    
                                
                                    
                                        
                                        Assign Leave
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        Leave List
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        Timesheets
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        Apply Leave
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        My Leave
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        My Timesheet
                                    
                                                        
                                                
                                        
                
        
    



    $(document).ready(function () {
        // hover color change effect
        $(&quot;#dashboard-quick-launch-panel-slider li&quot;).hover(function () {
            $(this).animate({opacity: 0.90}, 100, function () {
                $(this).animate({opacity: 1}, 0);
            });
        });
        // Trigger mouse move event over the &quot; , &quot;'&quot; , &quot;menu_holder&quot; , &quot;'&quot; , &quot;.
        $(&quot;#dashboard-quick-launch-panel-menu_holder&quot;).mousemove(function (e) {
            // Enable scroll function only when the height of the &quot; , &quot;'&quot; , &quot;slider&quot; , &quot;'&quot; , &quot; or menu is greater than the &quot; , &quot;'&quot; , &quot;menu_holder&quot; , &quot;'&quot; , &quot;.
            if ($(this).height() &lt; $(&quot;#dashboard-quick-launch-panel-slider&quot;).height()) {
                // Calculate the distance value from the &quot; , &quot;'&quot; , &quot;menu_holder&quot; , &quot;'&quot; , &quot; y pos and page Y pos.
                var distance = e.pageY - $(this).offset().top;
                // Get the percentage value with respect to the Mouse Y on the &quot; , &quot;'&quot; , &quot;menu_holder&quot; , &quot;'&quot; , &quot;.
                var percentage = distance / $(this).height();
                // Calculate the new Y position of the &quot; , &quot;'&quot; , &quot;slider&quot; , &quot;'&quot; , &quot;. 
                var targetY = -Math.round(($(&quot;#dashboard-quick-launch-panel-slider&quot;).height() - $(this).height()) * percentage);
                // With jQuery easing funtion from easing plugin.
                $(&quot; , &quot;'&quot; , &quot;#dashboard-quick-launch-panel-slider&quot; , &quot;'&quot; , &quot;).animate({top: [targetY + &quot;px&quot;, &quot;easeOutCirc&quot;]}, {queue: false, duration: 200});
                // Without easeing function. by default jQuery have &quot; , &quot;'&quot; , &quot;swing&quot; , &quot;'&quot; , &quot;.
                //$(&quot; , &quot;'&quot; , &quot;#slider&quot; , &quot;'&quot; , &quot;).animate({top: [targetY+&quot;px&quot;, &quot;easeOutCirc&quot;]}, { queue:false, duration:200 });
            }
        });
    });

                                 
                            

                                            
                
            
                        
                
                                        
                                                                                
                                
                                    Employee Distribution by Subunit
                                    
    






    69%10%
       
        
        
        

    
        $(document).ready(function () {

            var data = {&quot;Not assigned to Subunits&quot;:66,&quot;Administration&quot;:2,&quot;Client Services&quot;:3,&quot;Engineering&quot;:10,&quot;Finance&quot;:4,&quot;Human Resources&quot;:5,&quot;Sales &amp; Marketing&quot;:6};
            var graph = {&quot;legend&quot;:{&quot;labels&quot;:[&quot;Not assigned to Subunits&quot;,&quot;Administration&quot;,&quot;Client Services&quot;,&quot;Engineering&quot;,&quot;Finance&quot;,&quot;Human Resources&quot;,&quot;Sales &amp;amp; Marketing&quot;],&quot;noOfColumns&quot;:7,&quot;legendDivId&quot;:&quot;div_legend_pim_employee_distribution&quot;,&quot;useSeparateContainer&quot;:true}};


            var properties = {&quot;show-legend&quot;:true,&quot;show-labels&quot;:true,&quot;interactive&quot;:true,&quot;suffixForValueHover&quot;:&quot;Employee(s)&quot;};

            visualizePieChart(data, graph, properties, &quot; , &quot;'&quot; , &quot;div_graph_display_emp_distribution&quot; , &quot;'&quot; , &quot;);        });
       

    

        $(document).ready(function () {
            var moduleUrl = &quot; , &quot;'&quot; , &quot;/index.php/dashboard/employeeDistribution&quot; , &quot;'&quot; , &quot;;
            var divId = &quot; , &quot;'&quot; , &quot;dashboard__employeeDistribution&quot; , &quot;'&quot; , &quot;;
            $.ajax({
                url: moduleUrl,
                success: function (obj) {
                    $(&quot;#&quot; + divId).html(obj);
                },
                complete: function () {
                    $(&quot;#&quot; + divId).removeClass(&quot; , &quot;'&quot; , &quot;loadmask&quot; , &quot;'&quot; , &quot;);
                }
            });
        });
    
                                 
                            

                                                                                
                                
                                    Legend
                                    
    .task-list-group-panel-menu_holder{
    margin:0;
    padding: 5px;
    padding-top: 8px;
    /*background:white;*/
    overflow-y:auto; /* make the overflow as hidden, like a mask layer*/
    overflow-x: hidden;
    height:100%; /* Height of the menu holder, same as the container*/
    position:relative;
    /*width:100%;*/
}


    Not assigned to SubunitsAdministrationClient ServicesEngineeringFinanceHuman ResourcesSales &amp; Marketing
                                 
                            

                                                                                
                                
                                    Pending Leave Requests
                                    
    
    
        
            
                                        No Records are Available
                              
        
    
    
        
            
                 
                     3 month(s)                
                 
                    Total : 0 / 0                                
            
        
    



    $(document).ready(function() {
        // hover color change effect
        $(&quot;#task-list-group-panel-slider li&quot;).hover(function() {
            $(this).animate({opacity: 0.90}, 100, function(){ 
                $(this).animate({opacity: 1}, 0);
            } );
        });     
    });


    

        $(document).ready(function () {
            var moduleUrl = &quot; , &quot;'&quot; , &quot;/index.php/dashboard/pendingLeaveRequests&quot; , &quot;'&quot; , &quot;;
            var divId = &quot; , &quot;'&quot; , &quot;dashboard__pendingLeaveRequests&quot; , &quot;'&quot; , &quot;;
            $.ajax({
                url: moduleUrl,
                success: function (obj) {
                    $(&quot;#&quot; + divId).html(obj);
                },
                complete: function () {
                    $(&quot;#&quot; + divId).removeClass(&quot; , &quot;'&quot; , &quot;loadmask&quot; , &quot;'&quot; , &quot;);
                }
            });
        });
    
                                 
                            

                                            
                
            
                


             

        &quot;))]</value>
      <webElementGuid>edae019d-7450-468c-aa48-795fed4391fe</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
